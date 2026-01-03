use monitor::{
    MetricsReceiver,
    logging::Logger,
    receiver::{MockReceiver, Receiver},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr = "127.0.0.1:8080";

    println!(" Запуск системы мониторинга банковского хранилища");
    println!("Прослушивание адреса: {}", bind_addr);
    println!("──────────────────────────────────────────────────");

    // let receiver = MetricsReceiver::new(bind_addr)?;
    let receiver: Box<dyn Receiver> = if std::env::var("USE_MOCK").is_ok() {
        Box::new(MockReceiver)
    } else {
        Box::new(MetricsReceiver::new(bind_addr)?)
    };
    let mut loggers: Vec<Box<dyn Logger>> = vec![];
    let (receiver_handle, metrics_rx) = receiver.start_with_channel();

    println!("Система мониторинга запущена. Ожидание данных...");
    println!("Нажмите Ctrl+C для остановки");

    let mut total_received = 0;

    // Основной цикл обработки данных
    loop {
        match metrics_rx.recv() {
            Ok((metrics, _src_addr)) => {
                total_received += 1;

                // Определяем статус тревоги
                let alert_status = if metrics.door_open {
                    "🚨 ТРЕВОГА: ДВЕРЬ ОТКРЫТА!"
                } else if metrics.temperature > 30.0 {
                    "⚠️  ВНИМАНИЕ: Высокая температура"
                } else if metrics.humidity > 70.0 {
                    "⚠️  ВНИМАНИЕ: Высокая влажность"
                } else {
                    "✅ Норма"
                };

                println!(
                    "[#{:03}] {} | Темп: {:.1}°C | Влажн: {:.1}% | Давл: {:.1}hPa | Ур. шума: {:.1} | Дверь: {} | {}",
                    total_received,
                    metrics.formatted_time(),
                    metrics.temperature,
                    metrics.humidity,
                    metrics.pressure,
                    metrics.noise_level,
                    if metrics.door_open {
                        "ОТКРЫТА"
                    } else {
                        "закрыта"
                    },
                    alert_status
                );
            }
            Err(_) => {
                println!("🔌 Канал закрыт. Завершение работы.");
                break;
            }
        }
    }

    // Пытаемся дождаться завершения потока
    let _ = receiver_handle.join();

    println!("Итог: получено {} пакетов данных", total_received);
    Ok(())
}
