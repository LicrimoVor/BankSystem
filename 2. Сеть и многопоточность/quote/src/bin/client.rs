use clap::{Parser, command};
use quote::{
    client::ClientQuote,
    types::{message::UdpMessage, stock::Ticker},
};
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Тип экстрактора
    #[arg(short, long)]
    server: SocketAddr,

    /// Адрес udp клиента
    #[arg(long)]
    host: Option<SocketAddr>,

    /// Тикеры
    #[arg(short, long)]
    tickers: Vec<Ticker>,
}

fn main() {
    #[cfg(feature = "logging")]
    {
        use log::info;
        env_logger::init();
        info!("Логирование инициализировано");
    }

    let Cli {
        server,
        host,
        tickers,
    } = Cli::parse();

    let host = host.unwrap_or("127.0.0.1:7878".parse().unwrap());
    let Ok(mut client) = ClientQuote::new(server) else {
        println!("Init failed");
        return;
    };

    match client.get_tickers() {
        Ok(tickers) => {
            println!("All tickers in server: {:#?}", tickers);
        }
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let reciever = match client.create_reciever(tickers, host) {
        Ok(reciever) => reciever,
        Err(e) => {
            println!("Error create reciever: {:?}", e);
            return;
        }
    };

    loop {
        let msg = match reciever.recv() {
            Ok(msg) => msg,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };
        match msg {
            UdpMessage::Stock(stock) => {
                println!("{}", stock);
            }
            UdpMessage::Init(stocks) => {
                for stock in stocks {
                    println!("{}", stock);
                }
            }
            UdpMessage::Disconnect => {
                println!("Disconnect");
                return;
            }
            _ => (),
        }
    }
}
