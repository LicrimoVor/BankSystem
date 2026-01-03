use crate::{
    distributor::{Event, Subscriber},
    stock::{self, StockQuote, Ticker},
};
use std::thread;

enum Message {
    Hello(Vec<StockQuote>),
    Stock(StockQuote),
    Close(Ticker),
    Pong,
    Disconnect,
}

impl Message {
    fn to_string(&self) -> String {
        match self {
            Message::Hello(stocks) => {
                let mut mess = "Welcome! Last stocks (ticker|price|volume|timestamp):".to_string();
                for stock in stocks {
                    mess.push_str(format!("\n{}", stock.to_string()).as_str());
                }
                mess
            }
            Message::Stock(stock) => stock.to_string(),
            Message::Close(ticker) => format!("Close stock: {}", ticker),
            Message::Pong => "PONG".to_string(),
            Message::Disconnect => "Disconnect".to_string(),
        }
    }
}

struct ActorWorker {
    subscriber: Subscriber,
}

impl ActorWorker {
    pub fn new(subscriber: Subscriber) -> Self {
        Self { subscriber }
    }

    pub fn start(self, last_stocks: Vec<StockQuote>) -> thread::JoinHandle<()> {
        self.send(Message::Hello(last_stocks));

        thread::spawn(move || {
            loop {
                if let Ok(event) = self.subscriber.get_event() {
                    match event {
                        Event::Update(stock) => self.send(Message::Stock(stock)),
                        Event::Close(ticker) => self.send(Message::Close(ticker)),
                        Event::Disconnect => {
                            self.stop();
                            break;
                        }
                    }
                };
            }
        })
    }

    fn stop(self) {
        self.send(Message::Disconnect);
    }
    fn send(&self, message: Message) {}
}
