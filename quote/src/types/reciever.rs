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
        Arc, RwLock,
        mpsc::{Receiver, Sender},
    },
    thread,
};

const READ_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
const TIME_SLEEP: std::time::Duration = std::time::Duration::from_millis(1000);
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
    latency: u128,
    time_instant: Option<std::time::Instant>,

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
        if let Err(_e) = socket.set_read_timeout(Some(READ_TIMEOUT)) {
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
                latency: 1000,
                time_instant: None,

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
            let Ok(shutdown) = self.shutdown.read().map(|s| *s) else {
                logging!(warn, ("Не удалось получить shutdown"));
                continue;
            };

            if shutdown {
                self.message_handle(UdpMessage::Disconnect);
                break Ok(self);
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
                    self.keepalive();
                    self.count_timeout += 1;
                    if self.count_timeout >= COUNT_TIMEOUT {
                        self.reconnect();
                        logging!(warn, ("Reconnect"));
                    }
                }
                Err(_e) => {
                    self.reconnect();
                    self.keepalive();
                    logging!(warn, ("{}", _e));
                }
            }

            if self.count_reconnect >= COUNT_RECONNECT {
                break Err("Reconnect failed".to_string());
            }
        }
    }

    fn reconnect(&mut self) {
        self.count_reconnect += 1;
        self.socket = match UdpSocket::bind(self.addr) {
            Ok(s) => s,
            Err(_e) => {
                logging!(warn, ("{}", _e));
                return;
            }
        };
        if let Err(_e) = self.socket.set_read_timeout(Some(READ_TIMEOUT)) {
            logging!(warn, ("{}", _e));
        };

        if let Err(_e) = self.socket.set_nonblocking(true) {
            logging!(warn, ("{}", _e));
        };
    }

    fn message_handle(&mut self, message: UdpMessage) {
        self.count_reconnect = 0;
        self.count_timeout = 0;
        match message {
            UdpMessage::Init(stocks) => {
                self.sender.send(UdpMessage::Init(stocks)).unwrap();
            }
            UdpMessage::Stock(stock) => {
                self.sender.send(UdpMessage::Stock(stock)).unwrap();
            }
            UdpMessage::Disconnect => {
                self.sender.send(UdpMessage::Disconnect).unwrap();
            }
            UdpMessage::Ping => {
                if let Some(instant) = self.time_instant {
                    let latency = instant.elapsed().as_millis();
                    if latency > self.latency {
                        self.latency = latency;
                        logging!(info, ("Latency: {}", self.latency));
                    }
                }
            }
            _ => (),
        }
    }

    fn keepalive(&mut self) {
        self.time_instant = Some(std::time::Instant::now());

        if let Ok(ping) = UdpMessage::Ping.to_format(&self.format) {
            if let Err(_e) = self.socket.send_to(&ping, self.server.unwrap()) {
                logging!(warn, ("Ошибка отправки ping: {}", _e));
            };
        };
    }
}
