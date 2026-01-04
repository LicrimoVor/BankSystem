use crate::{
    distributor::{Event, Subscriber},
    message::Message,
    stock::{StockQuote, Ticker},
};
use std::{
    io::Error,
    net::{SocketAddr, UdpSocket},
    thread,
};

const DURATION_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(100);
const TIME_DISCONNET: std::time::Duration = std::time::Duration::from_secs(5);
const COUNT_RECONNECT: u32 = 5;

pub struct ActorWorker {
    subscriber: Subscriber,
    socket: UdpSocket,
}

impl ActorWorker {
    pub fn new(subscriber: Subscriber, addr: SocketAddr) -> Result<Self, String> {
        let Ok(socket) = UdpSocket::bind(addr) else {
            return Err("Не удалось создать сокет".to_string());
        };
        socket
            .set_read_timeout(Some(DURATION_TIMEOUT))
            .map_err(|_| "Не удалось установить таймаут".to_string())?;

        Ok(Self { subscriber, socket })
    }

    pub fn run(mut self, last_stocks: Vec<StockQuote>) -> Result<(), Error> {
        self.send(Message::Init(last_stocks));
        loop {
            if let Ok(event) = self.subscriber.get_event() {
                match event {
                    Event::Update(stock) => self.send(Message::Stock(stock)),
                    Event::Close(ticker) => self.send(Message::Close(ticker)),
                    Event::Disconnect => {
                        self.send(Message::Disconnect);
                        break Ok(());
                    }
                }
            };

            let mut buf = [0u8; 1024];
            match self.socket.recv(&mut buf) {
                Ok(received) => {}
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                Err(e) => {
                    println!("Error: {}", e);
                    break Err(e);
                }
            };
        }
    }

    fn send(&mut self, message: Message) {
        let _ = self.socket.send(message.to_string().as_bytes());
    }
}
