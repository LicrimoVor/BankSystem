use crate::stock::{StockQuote, Ticker};
use log::{info, warn};
use std::{
    collections::HashMap,
    rc::Rc,
    sync::mpsc::{Receiver, Sender, TryRecvError, channel},
};

pub enum Event {
    /// Обновить данные о акции
    Update(StockQuote),

    /// Закрыть подписку на эту акцию
    Close(Ticker),

    /// Отключить пользователя
    Disconnect,
}

pub struct Subscriber {
    id: u32,
    stocks: Vec<Ticker>,
    receiver: Receiver<Event>,
}

impl Subscriber {
    fn new(id: u32, stocks: Vec<String>, receiver: Receiver<Event>) -> Self {
        Self {
            id,
            stocks,
            receiver,
        }
    }

    pub fn get_event(&self) -> Result<Event, TryRecvError> {
        self.receiver.try_recv()
    }
}

struct StockSender(Rc<Sender<Event>>, Vec<Ticker>);

/// Уведомляет подписчиков о новых ценах на акции
pub struct Distributor {
    last_stocks: HashMap<Ticker, StockQuote>,
    subscribers: HashMap<u32, StockSender>,

    stock_senders: HashMap<Ticker, HashMap<u32, Rc<Sender<Event>>>>,

    /// Счетчик подписок (служит для генерации id)
    __count: u32,
}

impl Distributor {
    fn new() -> Self {
        Self {
            last_stocks: HashMap::new(),
            subscribers: HashMap::new(),

            stock_senders: HashMap::new(),
            __count: 0,
        }
    }

    /// Подписаться на отслеживание акции
    pub fn subscribe(&mut self, stocks: Vec<Ticker>) -> Subscriber {
        let id = self.__count;
        self.__count += 1;

        #[cfg(feature = "logging")]
        info!("Подписка на акции {:?}. id: {}", stocks, id);

        let (sender, receiver) = channel();
        let sender = Rc::new(sender);

        for stock in &stocks {
            self.stock_senders
                .entry(stock.clone())
                .or_default()
                .insert(id, sender.clone());
        }

        self.subscribers
            .insert(id, StockSender(sender, stocks.clone()));

        return Subscriber::new(id, stocks, receiver);
    }

    /// Отписаться от отслеживания акции
    pub fn unsubscribe(&mut self, id: u32) {
        #[cfg(feature = "logging")]
        info!("Отпика от акций id: {}", id);

        if let Some(StockSender(_, stocks)) = self.subscribers.remove(&id) {
            for stock in stocks {
                if let Some(sender) = self.stock_senders.get_mut(&stock) {
                    sender.remove(&id);
                }
            }
        }
    }

    /// Отправить новые данные о акции
    pub fn send_all(&mut self, stock: StockQuote) {
        #[cfg(feature = "logging")]
        dbg!("Отправляем акцию: {:?}", &stock);

        self.last_stocks.insert(stock.ticker.clone(), stock.clone());

        if let Some(senders) = self.stock_senders.get(&stock.ticker) {
            for (_, sender) in senders.iter() {
                let _ = sender.send(Event::Update(stock.clone()));
            }
        }
    }

    /// Получить последние данные о акциях
    pub fn get_last_stocks(&self, stocks: Vec<Ticker>) -> Vec<StockQuote> {
        let mut result = Vec::new();
        for stock in stocks {
            if let Some(last_stock) = self.last_stocks.get(&stock) {
                result.push(last_stock.clone());
            }
        }
        result
    }
}
