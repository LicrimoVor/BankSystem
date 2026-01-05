use crate::{
    distributor::Distributor,
    types::{
        command::Command,
        message::MessageFormat,
        stock::{StockQuote, Ticker},
    },
    worker::SubscribeWorker,
};
use log::{info, warn};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::mpsc::Receiver,
    thread,
    time::Duration,
};

/// Тип соединения: адрес, id подписчика, поток
struct Connection(SocketAddr, u32, thread::JoinHandle<Result<(), String>>);

/// Конфиг мастера
pub struct MasterConfig {
    secret_key: Vec<u8>,
    tcp_addr: SocketAddr,
}

impl Default for MasterConfig {
    fn default() -> Self {
        let secret_key = rand::random_iter()
            .take(32)
            .map(|a: u8| 32 + a / 4)
            .collect::<Vec<u8>>();
        if let Ok(w) = String::from_utf8(secret_key.clone()) {
            println!("Secret key: {}", w);
        } else {
            println!("Secret key (bytes): {:?}", secret_key);
        }

        Self {
            secret_key,
            tcp_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878),
        }
    }
}

/// Мастер сервера потоков
pub struct Master {
    connections: HashMap<SocketAddr, Vec<Connection>>,
    distributor: Distributor,
    rx_stock: Receiver<StockQuote>,

    shutdown: bool,
    config: MasterConfig,
}

impl Master {
    pub fn new(rx_stock: Receiver<StockQuote>, config: Option<MasterConfig>) -> Self {
        Self {
            connections: HashMap::new(),
            distributor: Distributor::new(),
            rx_stock,

            shutdown: false,
            config: config.unwrap_or_default(),
        }
    }

    /// Запуск мастера (можно в отдельном потоке)
    pub fn run(mut self) -> Result<Self, String> {
        let listener = TcpListener::bind(self.config.tcp_addr).unwrap();
        listener.set_nonblocking(true).unwrap();
        loop {
            if self.shutdown {
                break;
            }

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        if let Err(e) = self.tcp_handle(stream) {
                            #[cfg(feature = "logging")]
                            warn!("{}", e);
                        };
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        continue;
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }
            }
            if let Ok(stock) = self.rx_stock.try_recv() {
                self.distributor.send_all(stock);
            }
            thread::sleep(Duration::from_micros(100));
        }

        let domens: Vec<SocketAddr> = self.connections.keys().cloned().collect();
        let mut threads = Vec::new();

        for domen in domens {
            let Some(connections) = self.connections.remove(&domen) else {
                continue;
            };
            for Connection(_, id, thread) in connections {
                self.distributor.unsubscribe(id);
                threads.push((id, thread));
            }
        }

        for (id, thread) in threads {
            match thread.join() {
                Ok(Ok(())) => {
                    #[cfg(feature = "logging")]
                    info!("Поток {} успешно завершился", id);
                }
                Ok(Err(e)) => {
                    #[cfg(feature = "logging")]
                    warn!("Поток {} завершился с ошибкой: {}", id, e);
                }
                Err(e) => {
                    #[cfg(feature = "logging")]
                    warn!("Поток {} запаниковал - {:#?}", id, e);
                }
            }
        }

        Ok(self)
    }

    fn tcp_handle(&mut self, mut stream: TcpStream) -> Result<(), String> {
        stream.set_nonblocking(true).unwrap();
        let mut buf = String::new();
        let Ok(domen) = stream.peer_addr() else {
            return Err("Connection failed".to_string());
        };
        let res: Result<String, &str> = match stream.read_to_string(&mut buf) {
            Ok(_) => match Command::parse(buf.as_str()) {
                Ok(command) => match command {
                    Command::Stream((socket, tickers)) => {
                        self.command_stream(domen, (socket, tickers))
                    }
                    Command::Stop(socket) => self.command_stop(domen, socket),
                    Command::Disconnect => Ok(self.command_disconnect(domen)),
                    Command::List => Ok(self.command_list(domen)),
                    Command::Tickers => Ok(self.command_tickers()),
                    Command::Help => Ok(Self::command_help()),
                    Command::Shutdown(key) => self.command_shutdown(key),
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            },
            Err(e) => Err(&e.to_string()),
        };

        let answer = match res {
            Ok(res) => res,
            Err(e) => e.to_string(),
        };

        if let Err(e) = stream.write_all(answer.as_bytes()) {
            return Err(e.to_string());
        };

        Ok(())
    }

    // --- обработка команд с tcp ---

    fn command_stream(
        &mut self,
        domen: SocketAddr,
        (socket, tickers): (SocketAddr, Vec<Ticker>),
    ) -> Result<String, &str> {
        if let Some(connections) = self.connections.get_mut(&domen) {
            if let Some(indx) = connections.iter().position(|c| c.0 == socket) {
                if connections[indx].2.is_finished() {
                    connections.remove(indx);
                    return Err("Поток завершен");
                }
                return Err("Поток уже запущен");
            }
        }
        let last_stocks = self.distributor.get_last_stocks(&tickers);

        // пока формат захардкожен для соблюдения ТЗ, но если надо будет, то можно будет передавать через команду)
        let (id, subscriber) = self.distributor.subscribe(tickers, MessageFormat::Json);
        let Ok(worker) = SubscribeWorker::new(subscriber, socket) else {
            return Err("Не удалось создать поток");
        };

        let handle = std::thread::spawn(move || worker.run(last_stocks));
        self.connections
            .entry(socket)
            .or_default()
            .push(Connection(socket, id, handle));
        Ok("Запущен".to_string())
    }

    fn command_stop(&mut self, domen: SocketAddr, socket: SocketAddr) -> Result<String, &str> {
        let Some(connections) = self.connections.get_mut(&domen) else {
            return Err("Поток не запущен");
        };
        let Some(indx) = connections.iter().position(|c| c.0 == socket) else {
            return Err("Поток не запущен");
        };

        let Connection(_, id, handle) = connections.remove(indx);
        self.distributor.unsubscribe(id);
        // Вопрос такой, правильно ли так делать?)
        // по факту это может затормозить tcp-поток...
        // handle.join();
        Ok("Остановлен".to_string())
    }

    fn command_list(&self, domen: SocketAddr) -> String {
        let Some(connections) = self.connections.get(&domen) else {
            return "Потоков нет".to_string();
        };
        let mut res = String::new();
        for (i, Connection(socket, _, handle)) in connections.iter().enumerate() {
            res.push_str(&format!("{}:{}:{}\n", i, socket, !handle.is_finished()));
        }
        res
    }

    fn command_disconnect(&mut self, domen: SocketAddr) -> String {
        let Some(connections) = self.connections.remove(&domen) else {
            return "Отключено".to_string();
        };
        for Connection(_, id, handle) in connections {
            self.distributor.unsubscribe(id);
        }

        return "Отключено".to_string();
    }

    fn command_tickers(&self) -> String {
        self.distributor.get_tickers().join("\n")
    }

    fn command_help() -> String {
        "
Комманды:
stream <ip>:<port> <ticker,ticker...> - создать поток
stop <ip>:<port> - остановить поток
list - список подключений
disconnect - отключиться (завершив все потоки)
tickers - список тикеров
help - список комманд"
            .to_string()
    }

    fn command_shutdown(&mut self, key: String) -> Result<String, &str> {
        if key == String::from_utf8(self.config.secret_key.clone()).unwrap() {
            self.shutdown = true;
            return Ok("Успешно завершено".to_string());
        }
        Err("Неверный ключ")
    }
}
