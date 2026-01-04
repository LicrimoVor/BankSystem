use crate::{
    distributor::Distributor,
    message::MessageFormat,
    stock::{StockQuote, Ticker},
    worker::ActorWorker,
};
use std::{
    collections::HashMap,
    io::{self, Error, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    sync::mpsc::Receiver,
    thread::JoinHandle,
};

/// Команды для общения с tcp-мастером
#[derive(Debug, PartialEq)]
enum Command {
    /// STREAM <ip>:<port> <ticker,ticker...>
    /// создать поток
    Stream((SocketAddr, Vec<Ticker>)),

    /// STOP <ip>:<port>
    /// остановить поток
    Stop(SocketAddr),

    /// LIST
    /// список подключений
    List,

    /// DISCONNECT
    /// отключиться (завершив все потоки)
    Disconnect,

    /// TICKERS
    /// список тикеров
    Tickers,

    /// HELP
    /// выводит список команд
    Help,
}

impl Command {
    pub fn parse(s: &str) -> Result<Self, &str> {
        match s[0..4].to_uppercase().as_str() {
            "STRE" => {
                let parts: Vec<&str> = s.split(' ').collect();
                if parts.len() != 3 {
                    return Err(
                        "Неправильная команда стрима\nSTREAM <ip>:<port> <ticker,ticker...>",
                    );
                }
                let tickers: Vec<Ticker> = parts[2]
                    .split(',')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                let Ok(addr) = SocketAddr::from_str(parts[1]) else {
                    return Err(
                        "Неправильная команда стрима\nSTREAM <ip>:<port> <ticker,ticker...>",
                    );
                };

                Ok(Command::Stream((addr, tickers)))
            }
            "STOP" => {
                let parts: Vec<&str> = s.split(' ').collect();
                if parts.len() != 2 {
                    return Err("Неправильная команда стоп\nSTOP <ip>:<port>");
                }
                let Ok(addr) = SocketAddr::from_str(parts[1]) else {
                    return Err("Неправильная команда стоп\nSTOP <ip>:<port>");
                };
                Ok(Command::Stop(addr))
            }
            "LIST" => Ok(Command::Disconnect),
            "DISC" => Ok(Command::Disconnect),
            "TICK" => Ok(Command::Tickers),
            "HELP" => Ok(Command::Help),
            _ => Err("Неизсветная команда. Отправьте HELP"),
        }
    }
}

/// Тип соединения: адрес, id подписчика, поток
struct Connection(SocketAddr, u32, JoinHandle<Result<(), Error>>);
/// Мастер сервера потоков
struct Master {
    connections: HashMap<SocketAddr, Vec<Connection>>,
    distributor: Distributor,
    rx_stock: Receiver<StockQuote>,
}

impl Master {
    pub fn new(rx_stock: Receiver<StockQuote>) -> Self {
        Self {
            connections: HashMap::new(),
            distributor: Distributor::new(),
            rx_stock,
        }
    }

    pub fn run(mut self) -> Result<Self, String> {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        listener.set_nonblocking(true).unwrap();
        loop {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        self.tcp_handle(stream).unwrap();
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
        }
        Ok(self)
    }

    fn tcp_handle(&mut self, mut stream: TcpStream) -> Result<(), String> {
        let mut buf = String::new();
        let Ok(domen) = stream.peer_addr() else {
            return Err("Connection failed".to_string());
        };
        match stream.read_to_string(&mut buf) {
            Ok(_) => {
                println!("{}", buf);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn command_stream(
        &mut self,
        domen: SocketAddr,
        (socket, tickers): (SocketAddr, Vec<Ticker>),
    ) -> Result<(), &str> {
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
        let Ok(worker) = ActorWorker::new(subscriber, socket) else {
            return Err("Не удалось создать поток");
        };

        let handle = std::thread::spawn(move || worker.run(last_stocks));
        self.connections
            .entry(socket)
            .or_default()
            .push(Connection(socket, id, handle));
        Ok(())
    }

    fn command_stop(&mut self, domen: SocketAddr, socket: SocketAddr) -> Result<(), &str> {
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
        Ok(())
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

    fn command_disconnect(&mut self, domen: SocketAddr) {
        let Some(connections) = self.connections.remove(&domen) else {
            return;
        };
        for Connection(_, id, handle) in connections {
            self.distributor.unsubscribe(id);
            // Вопрос такой, правильно ли так делать?)
            // по факту это может затормозить tcp-поток...
            // handle.join();
        }
    }

    fn command_tickers(&self) -> String {
        self.distributor.get_tickers().join("\n")
    }

    fn command_help() -> &'static str {
        "
Комманды:
stream <ip>:<port> <ticker,ticker...> - создать поток
stop <ip>:<port> - остановить поток
list - список подключений
disconnect - отключиться (завершив все потоки)
tickers - список тикеров
help - список комманд"
    }
}
