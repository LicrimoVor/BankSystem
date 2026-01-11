use crate::{distributor::Distributor, tcp_worker::TcpWorker, types::stock::StockQuote};
#[cfg(feature = "logging")]
use log::{info, warn};
use std::{
    collections::HashMap,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
    sync::{Arc, Mutex, RwLock, mpsc::Receiver},
    thread::{self, JoinHandle},
    time::Duration,
};

/// Тип соединения: адрес, id подписчика, поток
pub(crate) struct Connection(
    pub(crate) SocketAddr,
    pub(crate) u32,
    pub(crate) thread::JoinHandle<Result<(), String>>,
);

/// Конфиг мастера
pub struct MasterConfig {
    secret_key: String,
    tcp_addr: SocketAddr,
}

impl MasterConfig {
    pub fn new(secret_key: Option<String>, tcp_addr: Option<SocketAddr>) -> Self {
        Self {
            secret_key: secret_key.unwrap_or_else(|| Self::gen_secret_key()),
            tcp_addr: tcp_addr
                .unwrap_or_else(|| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878)),
        }
    }

    /// Генерация секретного ключа
    pub fn gen_secret_key() -> String {
        let secret_key = rand::random_iter()
            .take(32)
            .map(|a: u8| 32 + a / 4)
            .collect::<Vec<u8>>();
        String::from_utf8(secret_key.clone()).unwrap_or("*Pa$$w0rd*".to_string())
    }
}

impl Default for MasterConfig {
    fn default() -> Self {
        Self {
            secret_key: Self::gen_secret_key(),
            tcp_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878),
        }
    }
}

/// Мастер сервера потоков
pub struct Master {
    rx_stock: Receiver<StockQuote>,
    tcp_threads: Vec<JoinHandle<Result<TcpWorker, String>>>,

    state: Arc<MasterState>,
    config: MasterConfig,
}

impl Master {
    pub fn new(rx_stock: Receiver<StockQuote>, config: Option<MasterConfig>) -> Self {
        let config: MasterConfig = config.unwrap_or_default();
        let state = Arc::new(MasterState {
            connections: Mutex::new(HashMap::new()),
            distributor: Mutex::new(Distributor::new()),
            shutdown: RwLock::new(false),

            secret_key: RwLock::new(config.secret_key.clone()),
        });

        Self {
            rx_stock,
            state,
            config,

            tcp_threads: Vec::new(),
        }
    }

    /// Запуск мастера (можно в отдельном потоке)
    pub fn run(mut self) -> Result<Self, String> {
        let listener = TcpListener::bind(self.config.tcp_addr).map_err(|e| e.to_string())?;
        listener.set_nonblocking(true).map_err(|e| e.to_string())?;

        loop {
            if *self.state.shutdown.read().unwrap() {
                break;
            }

            match listener.accept() {
                Ok((stream, _)) => {
                    if let Ok(tcp_worker) = TcpWorker::new(stream, Arc::clone(&self.state)) {
                        let tcp_thread = thread::spawn(move || tcp_worker.run());
                        self.tcp_threads.push(tcp_thread);
                    } else {
                        #[cfg(feature = "logging")]
                        warn!("Connection failed");
                    };
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_micros(100));
                }
                Err(_e) => {
                    #[cfg(feature = "logging")]
                    warn!("Connection failed: {}", _e.to_string());
                }
            }
            if let Ok(stock) = self.rx_stock.try_recv() {
                if let Ok(mut distributor) = self.state.distributor.lock() {
                    distributor.send_all(stock);
                }
            }
        }

        let mut all_connections = self.state.connections.lock().map_err(|e| e.to_string())?;
        let mut distributor = self.state.distributor.lock().map_err(|e| e.to_string())?;
        let domens: Vec<SocketAddr> = all_connections.keys().cloned().collect();
        let mut threads = Vec::new();

        for domen in domens {
            let Some(connections) = all_connections.remove(&domen) else {
                continue;
            };
            for Connection(_, id, thread) in connections {
                distributor.unsubscribe(id);
                threads.push((id, thread));
            }
        }

        for (_id, thread) in threads {
            match thread.join() {
                Ok(Ok(())) => {
                    #[cfg(feature = "logging")]
                    info!("Поток {} успешно завершился", _id);
                }
                Ok(Err(_e)) => {
                    #[cfg(feature = "logging")]
                    warn!("Поток {} завершился с ошибкой: {}", _id, _e);
                }
                Err(_e) => {
                    #[cfg(feature = "logging")]
                    warn!("Поток {} запаниковал - {:#?}", _id, _e);
                }
            }
        }

        drop(all_connections);
        drop(distributor);

        Ok(self)
    }
}
