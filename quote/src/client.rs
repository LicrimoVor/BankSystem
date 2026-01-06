use crate::types::{
    command::Command, error::QuoteError, message::Message, reciever::RecieverQuote, stock::Ticker,
};
#[cfg(feature = "logging")]
use log::error;
use log::info;
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpStream},
    sync::{Arc, RwLock, mpsc::Receiver},
    thread::{self, JoinHandle},
};

/// Клиент для получения котировок
pub struct ClientQuote {
    recievers: HashMap<u32, (JoinHandle<Result<RecieverQuote, String>>, Arc<RwLock<bool>>)>,
    writer: TcpStream,
    reader: BufReader<TcpStream>,

    count: u32,
}

impl ClientQuote {
    pub fn new(socket: SocketAddr) -> Result<Self, String> {
        let socket = TcpStream::connect(socket).map_err(|e| e.to_string())?;
        socket.set_nodelay(true).map_err(|e| e.to_string())?;

        let reader = BufReader::new(socket.try_clone().map_err(|e| e.to_string())?);

        #[cfg(feature = "logging")]
        info!("Connection succsessfully created");

        Ok(Self {
            recievers: HashMap::new(),
            writer: socket,
            reader,

            count: 0,
        })
    }

    /// Создать новый ресивер
    pub fn create_reciever(
        &mut self,
        tickers: Vec<Ticker>,
        addr: SocketAddr,
    ) -> Result<Receiver<Message>, QuoteError> {
        let id = self.count;
        self.count += 1;
        let shutdown = Arc::new(RwLock::new(false));
        let (reciever_quote, receiver) =
            RecieverQuote::new(tickers.clone(), addr, None, shutdown.clone()).map_err(|e| {
                #[cfg(feature = "logging")]
                error!("{}", e);
                QuoteError::Other(e.to_string())
            })?;
        let reciever_join = thread::spawn(move || reciever_quote.run());
        self.recievers.insert(id, (reciever_join, shutdown));

        if let Err(answ) = self.send_socket(Command::Stream((addr, tickers))) {
            #[cfg(feature = "logging")]
            error!("{:?}", answ);
            return Err(answ);
        };
        Ok(receiver)
    }

    pub fn stop_reciever(&mut self, id: u32) -> Result<(), String> {
        let Some((reciever_join, shutdown)) = self.recievers.remove(&id) else {
            return Err("Reciever not found".to_string());
        };
        let Ok(mut shutdown) = shutdown.write() else {
            return Err("Shutdown failed".to_string());
        };
        *shutdown = true;
        let receiver = reciever_join.join().map_err(|e| format!("{:?}", e))??;

        let command = Command::Stop(receiver.addr).to_string();
        self.writer
            .write_all(command.as_bytes())
            .map_err(|e| e.to_string())?;

        let mut buf = [0u8; 1024];
        self.reader.read(&mut buf).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn stop_all_recievers(&mut self) {}

    pub fn get_tickers(&mut self) -> Result<Vec<Ticker>, QuoteError> {
        match self.send_socket(Command::Tickers) {
            Ok(tickers) => Ok(tickers.trim().split('|').map(|s| s.to_string()).collect()),
            Err(e) => Err(e),
        }
    }

    fn send_socket(&mut self, command: Command) -> Result<String, QuoteError> {
        #[cfg(feature = "logging")]
        info!("Command: {:?}", command.to_string());

        self.writer
            .write_all(command.to_string().as_bytes())
            .map_err(|_| QuoteError::NotConnection)?;
        self.writer.flush().map_err(|_| QuoteError::NotConnection)?;

        let mut buf = String::new();
        self.reader
            .read_line(&mut buf)
            .map_err(|_| QuoteError::NotConnection)?;

        if buf.len() < 5 {
            return Err(QuoteError::Other("Empty answer".to_string()));
        }

        match QuoteError::from_string(&buf.trim()) {
            Ok(e) => Err(e),
            Err(_) => Ok(buf),
        }
    }
}
