use std::thread;

use quote::{
    extractor::{ConsoleExtractor, Extractor, RandomExtractor},
    master::Master,
};

fn main() {
    println!("----------");
    println!("| SERVER |");
    println!("----------");

    #[cfg(feature = "logging")]
    {
        use log::info;

        env_logger::init();
        info!("Логирование инициализировано");
    }

    let mut extractor = ConsoleExtractor::new();
    let rx_stock = extractor.subscribe();
    let master = Master::new(rx_stock, None);

    let th_extractor = thread::spawn(move || extractor.run());
    match master.run() {
        Ok(master) => println!("Master: Все прошло успешно!"),
        Err(e) => println!("{}", e),
    };

    match th_extractor.join() {
        Ok(Ok(extractor)) => println!("Extractor: Все прошло успешно!"),
        Ok(Err(e)) => println!("{}", e),
        Err(e) => println!("{:?}", e),
    };
}
