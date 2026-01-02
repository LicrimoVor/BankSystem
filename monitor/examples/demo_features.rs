use monitor::{RoomMetrics, debug, info, init_logger, warn};

fn main() {
    // Инициализируем логирование, если фича включена
    init_logger();

    println!("Демонстрация всех features");
    println!("=============================");

    info!("Начало демонстрации фич");

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
    info!("Активные фичи:");

    #[cfg(feature = "random")]
    println!("Фича 'random' активна");

    #[cfg(feature = "sqlite")]
    println!("Фича 'sqlite' активна");

    #[cfg(feature = "logging")]
    println!("Фича 'logging' активна");

    #[cfg(not(feature = "random"))]
    println!("Фича 'random' отключена");

    #[cfg(not(feature = "sqlite"))]
    println!("Фича 'sqlite' отключена");

    #[cfg(not(feature = "logging"))]
    println!("Фича 'logging' отключена");

    // Демонстрация фичи sqlite
    #[cfg(feature = "sqlite")]
    {
        debug!("Генерация SQL-запроса");
        println!("\n SQL-запрос:");
        println!("{}", metrics.to_sql());
    }

    info!("Демонстрация завершена");
}
