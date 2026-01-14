use crate::{
    distributor::{Event, Subscriber},
    logging,
    types::{
        message::{UdpMessage, UdpMessageFormat},
        stock::StockQuote,
    },
};
use std::net::{SocketAddr, UdpSocket};

const DURATION_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);
const DURATION_SLEEP: std::time::Duration = std::time::Duration::from_millis(100);
const PING_INTERVAL: std::time::Duration = std::time::Duration::from_millis(5000);
const COUNT_TRY_SEND: u8 = 10;

/// Worker по Udp подписки
pub(crate) struct UdpWorker {
    subscriber: Subscriber,
    socket: UdpSocket,
    addr: SocketAddr,
    format: UdpMessageFormat,

    count: u8,
    ping_interval: std::time::Instant,
}

impl UdpWorker {
    pub(crate) fn new(
        subscriber: Subscriber,
        addr: SocketAddr,
        format: Option<UdpMessageFormat>,
    ) -> Result<Self, String> {
        let Ok(socket) = UdpSocket::bind("0.0.0.0:0") else {
            return Err("Not bind udp socket".to_string());
        };
        socket
            .connect(addr)
            .map_err(|_| "Not connect".to_string())?;
        socket
            .set_read_timeout(Some(DURATION_TIMEOUT))
            .map_err(|_| "Not set timeout".to_string())?;

        logging!(info, ("SubscribeWorker running: {}", addr));
        let format = format.unwrap_or(UdpMessageFormat::Json);

        Ok(Self {
            subscriber,
            socket,
            addr,
            format,

            count: 0,
            ping_interval: std::time::Instant::now(),
        })
    }

    /// Запустить worker
    pub(crate) fn run(mut self, last_stocks: Vec<StockQuote>) -> Result<(), String> {
        self.send(UdpMessage::Init(last_stocks));
        loop {
            if self.count >= COUNT_TRY_SEND {
                logging!(warn, ("Client disconnected. Not response: {}", self.addr));
                break Err("Client not response".to_string());
            }

            if let Ok(event) = self.subscriber.get_event() {
                match event {
                    Event::Update(stock) => self.send(UdpMessage::Stock(stock)),
                    Event::Disconnect => {
                        self.send(UdpMessage::Disconnect);

                        logging!(info, ("SubscribeWorker Disconnect: {}", self.addr));

                        break Ok(());
                    }
                }
            };

            let mut buf = [0u8; 1024];
            if (std::time::Instant::now() - self.ping_interval) > PING_INTERVAL {
                logging!(
                    warn,
                    ("Client disconnected. Ping not response: {}", self.addr,)
                );
                break Err("Client disconnected".to_string());
            }

            // по 9 пункту пытался изменить match+match на and_then+match
            // по моему стало хуже )
            let _ = self
                .socket
                .recv(&mut buf)
                .map_err(|e| match e {
                    e if e.kind() == std::io::ErrorKind::TimedOut
                        || e.kind() == std::io::ErrorKind::WouldBlock =>
                    {
                        ()
                    }
                    _e => {
                        logging!(warn, ("Error connection: {}", _e));
                        self.count += 1;
                    }
                })
                .and_then(
                    |size| match UdpMessage::from_format(&buf[..size], &self.format) {
                        Ok(msg) => {
                            self.count = 0;
                            match msg {
                                UdpMessage::Ping => {
                                    logging!(info, ("Ping: {}", self.addr));
                                    self.ping_interval = std::time::Instant::now();
                                    Ok(self.send(UdpMessage::Pong))
                                }
                                UdpMessage::Pong => {
                                    logging!(info, ("Pong: {}", self.addr));
                                    self.ping_interval = std::time::Instant::now();
                                    Ok(self.send(UdpMessage::Ping))
                                }
                                _ => Ok(()),
                            }
                        }
                        Err(e) => {
                            logging!(warn, ("Error parse message: {}", e));
                            Err(())
                        }
                    },
                );
        }
    }

    /// Отправить сообщение
    fn send(&mut self, message: UdpMessage) {
        let Ok(mes) = message.to_format(&self.format) else {
            logging!(error, ("Error format message"));
            return;
        };

        if let Err(_e) = self.socket.send_to(mes.as_slice(), self.addr) {
            std::thread::sleep(DURATION_SLEEP);
            logging!(error, ("Error send message: {}", _e));
            self.count += 1;
        } else {
            self.count = 0;
        };
    }
}
