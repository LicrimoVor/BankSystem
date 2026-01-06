use crate::types::{
    message::{Message, MessageFormat},
    stock::Ticker,
};
#[cfg(feature = "logging")]
use log::{info, warn};
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
const TIME_SLEEP: std::time::Duration = std::time::Duration::from_millis(100);
const COUNT_RECONNECT: u8 = 5;
const COUNT_TIMEOUT: u8 = 10;

pub struct RecieverQuote {
    pub addr: SocketAddr,
    pub tickers: Vec<Ticker>,

    socket: UdpSocket,
    format: MessageFormat,
    sender: Sender<Message>,

    count_reconnect: u8,
    count_timeout: u8,
    latency: u128,
    time_instant: Option<std::time::Instant>,

    shutdown: Arc<RwLock<bool>>,
}

impl RecieverQuote {
    pub fn new(
        tickers: Vec<Ticker>,
        addr: SocketAddr,
        format: Option<MessageFormat>,
        shutdown: Arc<RwLock<bool>>,
    ) -> Result<(Self, Receiver<Message>), String> {
        let Ok(socket) = UdpSocket::bind(addr) else {
            #[cfg(feature = "logging")]
            warn!("Не удалось создать сокет");
            return Err("Не удалось создать сокет".to_string());
        };
        if let Err(e) = socket.set_read_timeout(Some(READ_TIMEOUT)) {
            #[cfg(feature = "logging")]
            warn!("{}", e);
        };

        if let Err(e) = socket.set_nonblocking(true) {
            #[cfg(feature = "logging")]
            warn!("{}", e);
        };

        let (sender, receiver) = std::sync::mpsc::channel();

        Ok((
            Self {
                addr,
                tickers,

                socket,
                format: format.unwrap_or(MessageFormat::Json),
                sender,

                count_reconnect: 0,
                count_timeout: 0,
                latency: 1000,
                time_instant: None,

                shutdown,
            },
            receiver,
        ))
    }

    pub fn run(mut self) -> Result<(Self), String> {
        loop {
            let mut buf = [0u8; 1024];
            let Ok(shutdown) = self.shutdown.read().map(|s| *s) else {
                #[cfg(feature = "logging")]
                warn!("Не удалось получить shutdown");
                continue;
            };

            if shutdown {
                self.message_handle(Message::Disconnect);
                break Ok(self);
            }

            match self.socket.recv(&mut buf) {
                Ok(n) => match Message::from_format(&buf[..n], &self.format) {
                    Ok(msg) => match msg {
                        Message::Ping => {
                            #[cfg(feature = "logging")]
                            warn!("Ping не ожидался");
                        }
                        Message::Pong => {
                            self.count_reconnect = 0;
                            self.count_timeout = 0;
                            if let Some(instant) = self.time_instant {
                                self.latency = instant.elapsed().as_millis();
                            }
                            #[cfg(feature = "logging")]
                            info!("Latency: {}", self.latency);
                        }
                        Message::Disconnect => {
                            self.message_handle(msg);
                            break Ok(self);
                        }
                        _ => self.message_handle(msg),
                    },
                    Err(e) => {
                        #[cfg(feature = "logging")]
                        warn!("{}", e);
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
                        #[cfg(feature = "logging")]
                        warn!("Reconnect");
                    }
                }
                Err(e) => {
                    self.reconnect();
                    self.keepalive();
                    #[cfg(feature = "logging")]
                    warn!("{}", e);
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
            Err(e) => {
                #[cfg(feature = "logging")]
                warn!("{}", e);
                return;
            }
        };
        if let Err(e) = self.socket.set_read_timeout(Some(READ_TIMEOUT)) {
            #[cfg(feature = "logging")]
            warn!("{}", e);
        };

        if let Err(e) = self.socket.set_nonblocking(true) {
            #[cfg(feature = "logging")]
            warn!("{}", e);
        };
    }

    fn message_handle(&mut self, message: Message) {
        self.count_reconnect = 0;
        self.count_timeout = 0;

        if let Err(e) = self.sender.send(message) {
            #[cfg(feature = "logging")]
            warn!("{}", e);
        }
    }

    fn keepalive(&mut self) {
        self.time_instant = Some(std::time::Instant::now());

        if let Ok(ping) = Message::Ping.to_bin() {
            if let Err(e) = self.socket.send(&ping) {
                #[cfg(feature = "logging")]
                warn!("{}", e);
            };
        };
    }
}
