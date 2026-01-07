use crate::{
    master::{Connection, MasterState},
    types::{command::TcpCommand, error::QuoteError, stock::Ticker},
    udp_worker::UdpWorker,
};
#[cfg(feature = "logging")]
use log::{info, warn};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};
use std::{net::SocketAddr, sync::Arc};

const DURATION_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
const DURATION_SLEEP: std::time::Duration = std::time::Duration::from_millis(100);
const COUNT_TRY_SEND: u8 = 10;

/// Worker TCP соединения
pub(crate) struct TcpWorker {
    stream: TcpStream,
    state: Arc<MasterState>,
    domen: SocketAddr,

    count: u8,
}

impl TcpWorker {
    pub(crate) fn new(stream: TcpStream, state: Arc<MasterState>) -> Result<Self, String> {
        stream
            .set_read_timeout(Some(DURATION_TIMEOUT))
            .map_err(|e| e.to_string())?;
        stream.set_nodelay(true).map_err(|e| e.to_string())?;

        let Ok(domen) = stream.peer_addr() else {
            #[cfg(feature = "logging")]
            warn!("Connection failed");

            return Err("Connection failed".to_string());
        };

        #[cfg(feature = "logging")]
        info!("Tcp worker created: {}", domen);

        Ok(Self {
            stream,
            state,
            domen,

            count: 0,
        })
    }

    /// Запускает tcp worker
    pub(crate) fn run(mut self) -> Result<Self, String> {
        #[cfg(feature = "logging")]
        info!("TcpWorker running: {}", self.domen);

        let mut reader = BufReader::new(self.stream.try_clone().map_err(|e| e.to_string())?);

        loop {
            if *self.state.shutdown.read().unwrap() || self.count > COUNT_TRY_SEND {
                let _ = self.stream.write_all("Finish\n".as_bytes());
                break Ok(self);
            }

            let mut buf = String::new();
            match reader.read_line(&mut buf) {
                Ok(0) => {
                    break Ok(self);
                }
                Ok(_) => {
                    self.count = 0;
                    let _ = self.tcp_handle(&mut buf);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(DURATION_SLEEP);
                    continue;
                }
                Err(e) => {
                    #[cfg(feature = "logging")]
                    warn!("Error read: {:?}", e);

                    self.count += 1;
                    let answ = QuoteError::InternalError.to_string();
                    let _ = self.stream.write_all(&answ.into_bytes());
                }
            };
        }
    }

    /// Обработка команд
    fn tcp_handle(&mut self, buffer: &mut String) -> Result<(), QuoteError> {
        #[cfg(feature = "logging")]
        info!("Command tcp: {:?}", buffer);
        let res: Result<String, QuoteError> = match TcpCommand::parse(buffer) {
            Ok(command) => match command {
                TcpCommand::Stream((socket, tickers)) => self.command_stream(socket, tickers),
                TcpCommand::Stop(socket) => self.command_stop(socket),
                TcpCommand::Disconnect => self.command_disconnect(),
                TcpCommand::List => self.command_list(),
                TcpCommand::Tickers => self.command_tickers(),
                TcpCommand::Help => Ok(Self::command_help()),
                TcpCommand::Shutdown(key) => self.command_shutdown(key),
            },
            Err(e) => Err(QuoteError::BadRequest(e.to_string())),
        };

        let answer = match res.clone() {
            Ok(res) => res,
            Err(e) => e.to_string(),
        };
        let answer = format!("{}\n", answer);

        if let Err(e) = self.stream.write_all(&answer.into_bytes()) {
            #[cfg(feature = "logging")]
            warn!("Error write: {}", e);

            return Err(QuoteError::NotConnection);
        };

        res.map(|_| ())
    }

    // --- обработка команд с tcp ---

    fn command_stream(
        &mut self,
        socket: SocketAddr,
        tickers: Vec<Ticker>,
    ) -> Result<String, QuoteError> {
        let mut all_connections = self
            .state
            .connections
            .lock()
            .map_err(|_| QuoteError::InternalError)?;
        let mut distributor = self
            .state
            .distributor
            .lock()
            .map_err(|_| QuoteError::InternalError)?;

        if let Some(connections) = all_connections.get_mut(&self.domen) {
            if let Some(indx) = connections.iter().position(|c| c.0 == socket) {
                if connections[indx].2.is_finished() {
                    connections.remove(indx);
                    return Err(QuoteError::BadRequest("Поток уже запущен".to_string()));
                }
                return Err(QuoteError::AlreadyExists);
            }
        }
        let last_stocks = distributor.get_last_stocks(&tickers);

        let (id, subscriber) = distributor.subscribe(tickers);

        // пока формат захардкожен для соблюдения ТЗ, но если надо будет, то можно будет передавать через команду)
        let worker = match UdpWorker::new(subscriber, socket, None) {
            Ok(worker) => worker,
            Err(e) => {
                #[cfg(feature = "logging")]
                warn!("{}: Connection failed: {}", socket, e.to_string());
                return Err(QuoteError::NotConnection);
            }
        };

        let handle = std::thread::spawn(move || worker.run(last_stocks));
        all_connections
            .entry(socket)
            .or_default()
            .push(Connection(socket, id, handle));
        Ok("Running".to_string())
    }

    fn command_stop(&mut self, socket: SocketAddr) -> Result<String, QuoteError> {
        let mut all_connections = self
            .state
            .connections
            .lock()
            .map_err(|_| QuoteError::InternalError)?;
        let mut distributor = self
            .state
            .distributor
            .lock()
            .map_err(|_| QuoteError::InternalError)?;

        let Some(connections) = all_connections.get_mut(&self.domen) else {
            return Err(QuoteError::NotFound);
        };
        let Some(indx) = connections.iter().position(|c| c.0 == socket) else {
            return Err(QuoteError::NotFound);
        };

        let Connection(_, id, _) = connections.remove(indx);
        distributor.unsubscribe(id);
        // Вопрос такой, правильно ли так делать?)
        // по факту это может затормозить tcp-поток...
        // handle.join();
        Ok("Stopped".to_string())
    }

    fn command_list(&self) -> Result<String, QuoteError> {
        let all_connections = self
            .state
            .connections
            .lock()
            .map_err(|_| QuoteError::InternalError)?;

        let Some(connections) = all_connections.get(&self.domen) else {
            return Err(QuoteError::NotFound);
        };
        let mut res = String::new();
        for (i, Connection(socket, _, handle)) in connections.iter().enumerate() {
            res.push_str(&format!("{}:{}:{}\n", i, socket, !handle.is_finished()));
        }
        Ok(res)
    }

    fn command_disconnect(&mut self) -> Result<String, QuoteError> {
        let mut all_connections = self
            .state
            .connections
            .lock()
            .map_err(|_| QuoteError::InternalError)?;
        let mut distributor = self
            .state
            .distributor
            .lock()
            .map_err(|_| QuoteError::InternalError)?;

        let Some(connections) = all_connections.remove(&self.domen) else {
            return Err(QuoteError::NotFound);
        };
        for Connection(_, id, _) in connections {
            distributor.unsubscribe(id);
        }

        return Ok("Disconnected".to_string());
    }

    fn command_tickers(&self) -> Result<String, QuoteError> {
        let distributor = self
            .state
            .distributor
            .lock()
            .map_err(|_| QuoteError::InternalError)?;

        if distributor.get_tickers().is_empty() {
            return Err(QuoteError::NotFound);
        }

        Ok(distributor.get_tickers().join("|"))
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

    fn command_shutdown(&mut self, key: String) -> Result<String, QuoteError> {
        let secret_key = self
            .state
            .secret_key
            .read()
            .map_err(|_| QuoteError::InternalError)?;
        if key == *secret_key {
            let mut shutdown = self
                .state
                .shutdown
                .write()
                .map_err(|_| QuoteError::InternalError)?;
            *shutdown = true;
            return Ok("Shutdown".to_string());
        }
        Err(QuoteError::KeyNotEqual)
    }
}
