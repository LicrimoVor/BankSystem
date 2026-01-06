use crate::{
    types::message::MessageFormat,
    types::stock::{StockQuote, Ticker},
};
use log::info;
use std::{
    collections::HashMap,
    rc::Rc,
    sync::{
        Arc,
        mpsc::{Receiver, Sender, TryRecvError, channel},
    },
};

/// Событие для подписчиков
pub(crate) enum Event {
    /// Обновить данные о акции
    Update(StockQuote),

    /// Отключить пользователя
    Disconnect,
}

/// Подписчик
pub(crate) struct Subscriber {
    id: u32,
    tickers: Vec<Ticker>,
    receiver: Receiver<Event>,
    message_format: MessageFormat,
}

impl Subscriber {
    fn new(
        id: u32,
        tickers: Vec<String>,
        message_format: MessageFormat,
        receiver: Receiver<Event>,
    ) -> Self {
        Self {
            id,
            tickers,
            receiver,
            message_format,
        }
    }

    pub fn get_event(&self) -> Result<Event, TryRecvError> {
        self.receiver.try_recv()
    }
}

/// Данные о подписчике - какнал отправки, список акций
struct SenderTicker(Arc<Sender<Event>>, Vec<Ticker>);

/// Уведомляет подписчиков о новых ценах на акции
pub(crate) struct Distributor {
    last_stocks: HashMap<Ticker, StockQuote>,
    subscribers: HashMap<u32, SenderTicker>,

    ticker_senders: HashMap<Ticker, HashMap<u32, Arc<Sender<Event>>>>,

    /// Счетчик подписок (служит для генерации id)
    __count: u32,
}

impl Distributor {
    pub fn new() -> Self {
        Self {
            last_stocks: HashMap::new(),
            subscribers: HashMap::new(),

            ticker_senders: HashMap::new(),
            __count: 0,
        }
    }

    /// Подписаться на отслеживание акции
    pub fn subscribe(
        &mut self,
        tickers: Vec<Ticker>,
        message_format: MessageFormat,
    ) -> (u32, Subscriber) {
        let id = self.__count;
        self.__count += 1;

        #[cfg(feature = "logging")]
        info!("Подписка на акции {:?}. id: {}", tickers, id);

        let (sender, receiver) = channel();
        let sender = Arc::new(sender);

        for ticker in &tickers {
            self.ticker_senders
                .entry(ticker.clone())
                .or_default()
                .insert(id, sender.clone());
        }

        self.subscribers
            .insert(id, SenderTicker(sender, tickers.clone()));

        return (id, Subscriber::new(id, tickers, message_format, receiver));
    }

    /// Отписаться от отслеживания акции
    pub fn unsubscribe(&mut self, id: u32) {
        #[cfg(feature = "logging")]
        info!("Отпика от акций id: {}", id);

        if let Some(SenderTicker(sender, stocks)) = self.subscribers.remove(&id) {
            let _ = sender.send(Event::Disconnect);
            for stock in stocks {
                if let Some(sender) = self.ticker_senders.get_mut(&stock) {
                    sender.remove(&id);
                }
            }
        }
    }

    /// Отправить новые данные о акции
    pub fn send_all(&mut self, stock: StockQuote) {
        #[cfg(feature = "logging")]
        info!("Отправляем акцию: {:?}", &stock);

        self.last_stocks.insert(stock.ticker.clone(), stock.clone());

        if let Some(senders) = self.ticker_senders.get(&stock.ticker) {
            for (_, sender) in senders.iter() {
                let _ = sender.send(Event::Update(stock.clone()));
            }
        }
    }

    /// Получить последние данные о акциях
    pub fn get_last_stocks(&self, stocks: &Vec<Ticker>) -> Vec<StockQuote> {
        let mut result = Vec::new();
        for stock in stocks {
            if let Some(last_stock) = self.last_stocks.get(stock) {
                result.push(last_stock.clone());
            }
        }
        result
    }

    pub fn get_tickers(&self) -> Vec<Ticker> {
        self.last_stocks.keys().cloned().collect()
    }
}
