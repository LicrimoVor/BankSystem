use crate::types::stock::{StockQuote, Ticker};
use crossbeam::channel;
#[cfg(feature = "logging")]
use log::info;
use std::{collections::HashMap, sync::Arc};

/// Событие для подписчиков
pub(crate) enum Event {
    /// Обновить данные о акции
    Update(StockQuote),

    /// Отключить пользователя
    Disconnect,
}

/// Udp Подписчик
pub(crate) struct Subscriber {
    _id: u32,
    _tickers: Vec<Ticker>,
    receiver: channel::Receiver<Event>,
}

impl Subscriber {
    fn new(id: u32, tickers: Vec<String>, receiver: channel::Receiver<Event>) -> Self {
        Self {
            _id: id,
            _tickers: tickers,
            receiver,
        }
    }

    pub fn get_event(&self) -> Result<Event, channel::TryRecvError> {
        self.receiver.try_recv()
    }
}

/// Данные о подписчике - какнал отправки, список акций
struct SenderTicker(Arc<channel::Sender<Event>>, Vec<Ticker>);

/// Уведомляет подписчиков о новых ценах на акции
pub(crate) struct Distributor {
    last_stocks: HashMap<Ticker, StockQuote>,
    subscribers: HashMap<u32, SenderTicker>,

    /// ticker: {id: sender}
    ticker_senders: HashMap<Ticker, HashMap<u32, Arc<channel::Sender<Event>>>>,

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
    pub fn subscribe(&mut self, tickers: Vec<Ticker>) -> (u32, Subscriber) {
        let id = self.__count;
        self.__count += 1;

        #[cfg(feature = "logging")]
        info!("Подписка на акции {:?}. id: {}", tickers, id);

        let (sender, receiver) = channel::bounded(1024);
        let sender = Arc::new(sender);

        for ticker in &tickers {
            self.ticker_senders
                .entry(ticker.clone())
                .or_default()
                .insert(id, sender.clone());
        }

        self.subscribers
            .insert(id, SenderTicker(sender, tickers.clone()));

        return (id, Subscriber::new(id, tickers, receiver));
    }

    /// Отписаться от отслеживания акции
    pub fn unsubscribe(&mut self, id: u32) {
        #[cfg(feature = "logging")]
        info!("Отпика от акций id: {}", id);

        if let Some(SenderTicker(sender, tickers)) = self.subscribers.remove(&id) {
            let _ = sender.send(Event::Disconnect);
            for ticker in tickers {
                // получаем всех senders, которые соответствуют этой акции и этому пользователю
                if let Some(senders) = self.ticker_senders.get_mut(&ticker) {
                    senders.remove(&id);
                }
                // если на такой тикер никто не подписан, удаляем его
                if let Some(senders) = self.ticker_senders.get(&ticker) {
                    if senders.is_empty() {
                        self.ticker_senders.remove(&ticker);
                    }
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
