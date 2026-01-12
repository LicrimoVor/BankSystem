use clap::{Parser, command};
use quote::{
    extractor::{ConsoleExtractor, Extractor, ExtractorType, FileMockExtractor, RandomExtractor},
    master::{Master, MasterConfig},
};
use std::{fs::File, net::SocketAddr, thread};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Тип экстрактора
    #[arg(short, long)]
    extractor: ExtractorType,

    /// Файл для экстратора
    #[arg(short, long, required_if_eq("extractor", "file"))]
    file: Option<String>,

    /// Адрес сервера
    #[arg(long)]
    host: Option<SocketAddr>,

    /// Секретный ключ
    #[arg(short, long)]
    secret_key: Option<String>,
}

fn main() {
    #[cfg(feature = "logging")]
    {
        use log::info;

        env_logger::init();
        info!("Логирование инициализировано");
    }

    let Cli {
        extractor,
        file,
        host,
        secret_key,
    } = Cli::parse();

    let file: Option<File> = if extractor == ExtractorType::File {
        let file = file.unwrap();
        if !std::path::Path::new(&file).exists() {
            println!("Файл не существует");
            return;
        }

        let Ok(file) = File::open(file) else {
            println!("Невозможно открыть файл");
            return;
        };

        Some(file)
    } else {
        None
    };

    let mut extractor: Box<dyn Extractor> = match extractor {
        ExtractorType::Console => Box::new(ConsoleExtractor::new()),
        ExtractorType::File => Box::new(FileMockExtractor::new(file.unwrap())),
        ExtractorType::Random => Box::new(RandomExtractor::new()),
        _ => Box::new(RandomExtractor::new()),
    };

    let rx_stock = extractor.subscribe();
    let config = MasterConfig::new(secret_key, host);
    let master = Master::new(rx_stock, Some(config));

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
