use crate::{
    logging,
    types::{
        command::TcpCommand, error::QuoteError, message::UdpMessage, reciever::RecieverQuote,
        stock::Ticker,
    },
};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
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
        logging!(info, ("Connection succsessfully created"));

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
    ) -> Result<Receiver<UdpMessage>, QuoteError> {
        let id = self.count;
        self.count += 1;
        let shutdown = Arc::new(RwLock::new(false));
        let (reciever_quote, receiver) =
            RecieverQuote::new(tickers.clone(), addr, None, shutdown.clone()).map_err(|e| {
                logging!(error, ("Error create reciever: {}", e));

                QuoteError::Other(e.to_string())
            })?;
        let reciever_join = thread::spawn(move || reciever_quote.run());

        if let Err(answ) = self.send_socket(TcpCommand::Stream((addr, tickers))) {
            logging!(debug, ("Error create reciever: {:?}", answ));
            let mut shutdown = shutdown.write().unwrap();
            *shutdown = true;

            match reciever_join.join() {
                Ok(Ok(_)) => {}
                Ok(Err(_e)) => {
                    logging!(error, ("{:?}", _e));
                }
                Err(_e) => {
                    logging!(error, ("{:?}", _e));
                }
            };

            return Err(answ);
        };
        self.recievers.insert(id, (reciever_join, shutdown));

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

        self.send_socket(TcpCommand::Stop(receiver.addr))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn get_tickers(&mut self) -> Result<Vec<Ticker>, QuoteError> {
        match self.send_socket(TcpCommand::Tickers) {
            Ok(tickers) => Ok(tickers.trim().split('|').map(|s| s.to_string()).collect()),
            Err(e) => Err(e),
        }
    }

    fn send_socket(&mut self, command: TcpCommand) -> Result<String, QuoteError> {
        logging!(info, ("Command {:?}", command.to_string()));

        self.writer
            .write_all(command.to_string().as_bytes())
            .map_err(|_| QuoteError::NotConnection)?;
        self.writer.flush().map_err(|_| QuoteError::NotConnection)?;

        let mut buf = String::new();
        self.reader
            .read_line(&mut buf)
            .map_err(|_| QuoteError::NotConnection)?;

        match QuoteError::from_string(&buf.trim()) {
            Ok(e) => Err(e),
            Err(_) => Ok(buf),
        }
    }
}
