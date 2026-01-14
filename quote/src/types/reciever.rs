use parking_lot::RwLock;

use crate::{
    logging,
    types::{
        message::{UdpMessage, UdpMessageFormat},
        stock::Ticker,
    },
};
use std::{
    io::{self},
    net::{SocketAddr, UdpSocket},
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
    thread,
};

const DURATION_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(200);
const TIME_SLEEP: std::time::Duration = std::time::Duration::from_millis(200);
const PING_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);
const COUNT_RECONNECT: u8 = 5;
const COUNT_TIMEOUT: u8 = 10;

pub(crate) struct RecieverQuote {
    pub(crate) addr: SocketAddr,
    _tickers: Vec<Ticker>,

    socket: UdpSocket,
    format: UdpMessageFormat,
    sender: Sender<UdpMessage>,
    server: Option<SocketAddr>,

    count_reconnect: u8,
    count_timeout: u8,
    ping_interval: std::time::Instant,

    shutdown: Arc<RwLock<bool>>,
}

impl RecieverQuote {
    pub(crate) fn new(
        tickers: Vec<Ticker>,
        addr: SocketAddr,
        format: Option<UdpMessageFormat>,
        shutdown: Arc<RwLock<bool>>,
    ) -> Result<(Self, Receiver<UdpMessage>), String> {
        let Ok(socket) = UdpSocket::bind(addr) else {
            logging!(warn, ("Не удалось создать сокет"));
            return Err("Не удалось создать сокет".to_string());
        };
        if let Err(_e) = socket.set_read_timeout(Some(DURATION_TIMEOUT)) {
            logging!(warn, ("Не удалось установить таймаут: {}", _e));
        };

        if let Err(_e) = socket.set_nonblocking(false) {
            logging!(warn, ("Не удалось установить nonblocking: {}", _e));
        };
        let (sender, receiver) = std::sync::mpsc::channel();
        let format = format.unwrap_or(UdpMessageFormat::Json);

        Ok((
            Self {
                addr,
                _tickers: tickers,

                socket,
                format,
                sender,
                server: None,

                count_reconnect: 0,
                count_timeout: 0,
                ping_interval: std::time::Instant::now(),

                shutdown,
            },
            receiver,
        ))
    }

    pub fn run(mut self) -> Result<Self, String> {
        let mut buf = [0u8; 1024];
        self.server = match self.socket.recv_from(&mut buf) {
            Ok((n, server)) => match UdpMessage::from_format(&buf[..n], &self.format) {
                Ok(msg) => {
                    if let UdpMessage::Init(last_stocks) = msg {
                        let _ = self.sender.send(UdpMessage::Init(last_stocks));
                        Some(server)
                    } else {
                        logging!(warn, ("Не удалось получить данные"));
                        return Err("Не удалось получить данные".to_string());
                    }
                }
                Err(e) => {
                    logging!(warn, ("Не удалось получить данные: {}", e));
                    return Err(e.to_string());
                }
            },
            Err(e) => {
                logging!(warn, ("Не удалось получить данные: {}", e));
                return Err(e.to_string());
            }
        };

        loop {
            let mut buf = [0u8; 1024];
            if *self.shutdown.read() {
                self.message_handle(UdpMessage::Disconnect);
                break Ok(self);
            }

            if (std::time::Instant::now() - self.ping_interval) > PING_INTERVAL {
                self.keepalive();
                self.count_timeout += 1;
            }

            match self.socket.recv(&mut buf) {
                Ok(n) => match UdpMessage::from_format(&buf[..n], &self.format) {
                    Ok(msg) => self.message_handle(msg),
                    Err(_e) => {
                        logging!(warn, ("Ошибка преобразования: {}", _e));
                    }
                },
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == io::ErrorKind::TimedOut =>
                {
                    thread::sleep(TIME_SLEEP);
                    self.count_timeout += 1;
                    if self.count_timeout >= COUNT_TIMEOUT {
                        self.reconnect();
                        logging!(warn, ("Reconnect"));
                    }
                }
                Err(_e) => {
                    self.reconnect();
                    logging!(warn, ("{}", _e));
                }
            }

            if self.count_reconnect >= COUNT_RECONNECT {
                logging!(warn, ("Server not response. Client closed"));
                break Err("Reconnect failed".to_string());
            }
        }
    }

    fn reconnect(&mut self) {
        self.count_reconnect += 1;
        self.socket = match UdpSocket::bind(self.addr) {
            Ok(s) => s,
            Err(_e) => {
                logging!(warn, ("Recconect failed: {}", _e));
                return;
            }
        };
        if let Err(_e) = self.socket.set_read_timeout(Some(DURATION_TIMEOUT)) {
            logging!(warn, ("Не удалось установить таймаут: {}", _e));
        };

        if let Err(_e) = self.socket.set_nonblocking(true) {
            logging!(warn, ("Не удалось установить nonblocking: {}", _e));
        };
    }

    fn message_handle(&mut self, message: UdpMessage) {
        self.count_reconnect = 0;
        self.count_timeout = 0;
        match message {
            UdpMessage::Init(stocks) => {
                if let Err(e) = self.sender.send(UdpMessage::Init(stocks)) {
                    logging!(warn, ("Send init failed: {}", e));
                };
            }
            UdpMessage::Stock(stock) => {
                if let Err(e) = self.sender.send(UdpMessage::Stock(stock)) {
                    logging!(warn, ("Send stock failed: {}", e));
                };
            }
            UdpMessage::Disconnect => {
                if let Err(e) = self.sender.send(UdpMessage::Disconnect) {
                    logging!(warn, ("Send disconnect failed: {}", e));
                };
            }
            UdpMessage::Pong => {
                logging!(info, ("Pong recv: {}", self.addr));
                self.ping_interval = std::time::Instant::now();
            }
            UdpMessage::Ping => {
                logging!(info, ("Ping recv: {}", self.addr));
                self.ping_interval = std::time::Instant::now();
            }
            _ => (),
        }
    }

    fn keepalive(&mut self) {
        if let (Some(server), Ok(ping)) = (self.server, UdpMessage::Ping.to_format(&self.format)) {
            logging!(info, ("Ping send: {}", self.addr));
            if let Err(_e) = self.socket.send_to(&ping, server) {
                logging!(warn, ("Ошибка отправки ping: {}", _e));
            };
        };
    }
}
