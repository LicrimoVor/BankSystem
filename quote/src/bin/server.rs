use std::thread;

use quote::{
    extractor::{ConsoleExtractor, Extractor, RandomExtractor},
    master::Master,
};

fn main() {
    #[cfg(feature = "logging")]
    {
        use log::info;

        env_logger::init();
        info!("Логирование инициализировано");
    }

    let mut extractor = RandomExtractor::new();
    let rx_stock = extractor.subscribe();
    let master = Master::new(rx_stock, None);

    let th_extractor = thread::spawn(move || extractor.run());
    match master.run() {
        Ok(_) => println!("Master: Все прошло успешно!"),
        Err(e) => println!("{}", e),
    };

    match th_extractor.join() {
        Ok(Ok(_)) => println!("Extractor: Все прошло успешно!"),
        Ok(Err(e)) => println!("{}", e),
        Err(e) => println!("{:?}", e),
    };
}
