use log::{error, info};
use quote::client::ClientQuote;

fn main() {
    #[cfg(feature = "logging")]
    {
        use log::info;

        env_logger::init();
        info!("Логирование инициализировано");
    }

    let Ok(mut client) = ClientQuote::new("127.0.0.1:7878".parse().unwrap()) else {
        println!("Init failed");
        return;
    };

    let tickers = match client.get_tickers() {
        Ok(tickers) => {
            println!("Tickers: {:#?}", tickers);
            tickers
        }
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let addr = "127.0.0.1:7888".parse().unwrap();
    let reciever = match client.create_reciever(tickers[0..2].to_vec(), addr) {
        Ok(reciever) => reciever,
        Err(e) => {
            info!("{:?}", e);
            return;
        }
    };

    for _ in 0..100 {
        let msg = match reciever.recv() {
            Ok(msg) => msg,
            Err(e) => {
                info!("{:?}", e);
                return;
            }
        };
        println!("{:?}", msg);
    }
}
