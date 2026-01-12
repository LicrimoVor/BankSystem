use crate::{
    master::Connection,
    types::{
        command::TcpCommand,
        error::QuoteError,
        state::{MasterState, MasterStateShell},
        stock::Ticker,
    },
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
        let mut shutdown = false;
        let shell = MasterStateShell::new(self.state.clone());
        loop {
            if let Ok(shutdown_guard) = shell.shutdown() {
                if **shutdown_guard.get() {
                    shutdown = true;
                }
            }
            if shutdown {
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
                    let _ = self.tcp_handle(&mut buf, &shell);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(DURATION_SLEEP);
                    continue;
                }
                Err(_e) => {
                    #[cfg(feature = "logging")]
                    warn!("Error read: {:?}", _e);

                    self.count += 1;
                    let answ = QuoteError::InternalError.to_string();
                    let _ = self.stream.write_all(&answ.into_bytes());
                }
            };
        }
    }

    /// Обработка команд
    fn tcp_handle(
        &mut self,
        buffer: &mut String,
        shell: &MasterStateShell,
    ) -> Result<(), QuoteError> {
        #[cfg(feature = "logging")]
        info!("Command tcp: {:?}", buffer);
        let res: Result<String, QuoteError> = match TcpCommand::parse(buffer) {
            Ok(command) => match command {
                TcpCommand::Stream((socket, tickers)) => {
                    self.command_stream(socket, tickers, &shell)
                }
                TcpCommand::Stop(socket) => self.command_stop(socket, &shell),
                TcpCommand::Disconnect => self.command_disconnect(&shell),
                TcpCommand::List => self.command_list(&shell),
                TcpCommand::Tickers => self.command_tickers(&shell),
                TcpCommand::Help => Ok(Self::command_help()),
                TcpCommand::Shutdown(key) => self.command_shutdown(key, &shell),
            },
            Err(e) => Err(QuoteError::BadRequest(e.to_string())),
        };

        let answer = match res.clone() {
            Ok(res) => res,
            Err(e) => e.to_string(),
        };
        let answer = format!("{}\n", answer);

        if let Err(_e) = self.stream.write_all(&answer.into_bytes()) {
            #[cfg(feature = "logging")]
            warn!("Error write: {}", _e);

            return Err(QuoteError::NotConnection);
        };

        res.map(|_| ())
    }

    // --- обработка команд с tcp ---

    fn command_stream(
        &mut self,
        socket: SocketAddr,
        tickers: Vec<Ticker>,
        shell: &MasterStateShell,
    ) -> Result<String, QuoteError> {
        let (Ok(mut all_connections_guard), Ok(mut distributor_guard)) =
            (shell.connections(), shell.distributor())
        else {
            return Err(QuoteError::InternalError);
        };
        let all_connections = all_connections_guard.get_mut();
        let distributor = distributor_guard.get_mut();

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
            Err(_e) => {
                #[cfg(feature = "logging")]
                warn!("{}: Connection failed: {}", socket, _e.to_string());
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

    fn command_stop(
        &mut self,
        socket: SocketAddr,
        shell: &MasterStateShell,
    ) -> Result<String, QuoteError> {
        let (Ok(mut all_connections_guard), Ok(mut distributor_guard)) =
            (shell.connections(), shell.distributor())
        else {
            return Err(QuoteError::InternalError);
        };
        let all_connections = all_connections_guard.get_mut();
        let distributor = distributor_guard.get_mut();

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

    fn command_list(&self, shell: &MasterStateShell) -> Result<String, QuoteError> {
        let Ok(all_connections_guard) = shell.connections() else {
            return Err(QuoteError::InternalError);
        };
        let Some(connections) = all_connections_guard.get().get(&self.domen) else {
            return Err(QuoteError::NotFound);
        };
        let mut res = String::new();
        for (i, Connection(socket, _, handle)) in connections.iter().enumerate() {
            res.push_str(&format!("{}:{}:{}\n", i, socket, !handle.is_finished()));
        }
        Ok(res)
    }

    fn command_disconnect(&mut self, shell: &MasterStateShell) -> Result<String, QuoteError> {
        let (Ok(mut all_connections_guard), Ok(mut distributor_guard)) =
            (shell.connections(), shell.distributor())
        else {
            return Err(QuoteError::InternalError);
        };

        let Some(connections) = all_connections_guard.get_mut().remove(&self.domen) else {
            return Err(QuoteError::NotFound);
        };
        for Connection(_, id, _) in connections {
            distributor_guard.get_mut().unsubscribe(id);
        }

        return Ok("Disconnected".to_string());
    }

    fn command_tickers(&self, shell: &MasterStateShell) -> Result<String, QuoteError> {
        let Ok(distributor_guard) = shell.distributor() else {
            return Err(QuoteError::InternalError);
        };
        let distributor = distributor_guard.get();
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

    fn command_shutdown(
        &mut self,
        key: String,
        shell: &MasterStateShell,
    ) -> Result<String, QuoteError> {
        let Ok(secret_key_guard) = shell.secret_key() else {
            return Err(QuoteError::InternalError);
        };

        if key == **secret_key_guard.get() {
            drop(secret_key_guard);

            let Ok(mut shutdown_guard) = shell.shutdown_mut() else {
                return Err(QuoteError::InternalError);
            };

            let shutdown = shutdown_guard.get_mut();
            **shutdown = true;
            return Ok("Shutdown".to_string());
        }
        Err(QuoteError::KeyNotEqual)
    }
}
