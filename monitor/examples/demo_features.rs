use monitor::{
    RoomMetrics, debug, info,
    logging::{ConsoleLogger, Logger, MemoryLogger, init_logger},
    warn,
};

fn main() {
    // Инициализируем логирование, если фича включена
    init_logger();
    let mut loggers: Vec<Box<dyn Logger>> = vec![
        Box::new(MemoryLogger::new(vec![])),
        Box::new(ConsoleLogger::new(vec![])),
        Box::new(ConsoleLogger::new(vec![Box::new(|m: String| {
            m.to_uppercase()
        })])),
    ];
    let mut log = |m| {
        for logger in loggers.iter_mut() {
            logger.log(m);
        }
    };

    println!("Демонстрация всех features");
    println!("=============================");

    log("Начало демонстрации фич");

    // Генерируем тестовые метрики
    debug!("Генерация тестовых метрик");
    let metrics = RoomMetrics::random();

    println!("Сгенерированные метрики:");
    println!("  Температура: {:.1}°C", metrics.temperature);
    println!("  Влажность: {:.1}%", metrics.humidity);
    println!("  Давление: {:.1}hPa", metrics.pressure);
    println!(
        "  Дверь: {}",
        if metrics.door_open {
            "открыта"
        } else {
            "закрыта"
        }
    );

    // Логирование в зависимости от состояния
    if metrics.door_open {
        warn!("Обнаружена открытая дверь!");
    }
    if metrics.temperature > 25.0 {
        warn!("Температура выше комфортной: {:.1}°C", metrics.temperature);
    }

    // Показываем, какие фичи активны
    log("Активные фичи:");

    #[cfg(feature = "random")]
    log("Фича 'random' активна");

    #[cfg(feature = "sqlite")]
    log("Фича 'sqlite' активна");

    #[cfg(feature = "logging")]
    log("Фича 'logging' активна");

    #[cfg(not(feature = "random"))]
    log("Фича 'random' отключена");

    #[cfg(not(feature = "sqlite"))]
    log("Фича 'sqlite' отключена");

    #[cfg(not(feature = "logging"))]
    log("Фича 'logging' отключена");

    // Демонстрация фичи sqlite
    #[cfg(feature = "sqlite")]
    {
        debug!("Генерация SQL-запроса");
        println!("\n SQL-запрос:");
        println!("{}", metrics.to_sql());
    }

    info!("Демонстрация завершена");
}
