use crate::RoomMetrics;
use crate::{debug, error, info, trace, warn};
use bincode;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

pub struct MetricsSender {
    socket: UdpSocket,
}

impl MetricsSender {
    pub fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(bind_addr)?;
        Ok(Self { socket })
    }

    // Метод отправки сообщений в сокет
    pub fn send_to(
        &self,
        metrics: &RoomMetrics,
        target_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Сериализация метрик: {:?}", metrics);
        let encoded = bincode::serialize(metrics)?;

        debug!("Отправка {} байт на {}", encoded.len(), target_addr);
        let _ = self.socket.send_to(&encoded, target_addr)?;

        trace!("Успешно отправлено {} байт", sent_bytes);
        Ok(())
    }

    // Метод для запуска цикла постоянной отправки метрик
    pub fn start_broadcasting(
        self,
        target_addr: String,
        interval_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Имитатор датчиков запущен. Отправка на {} каждые {}ms",
            target_addr, interval_ms
        );

        #[cfg(feature = "random")]
        info!("✅ Фича 'random' активна - используется rand для генерации данных");

        #[cfg(not(feature = "random"))]
        warn!("ℹ️  Фича 'random' отключена - используется детерминистическая генерация");

        info!(
            "Запуск трансляции метрик на {} каждые {} мс",
            target_addr, interval_ms
        );

        loop {
            let metrics = RoomMetrics::random();

            match self.send_to(&metrics, &target_addr) {
                Ok(()) => {
                    info!(
                        "[{}] Отправлено: {:.1}C, {:.1}% влажности, давление: {:.1}hPa, дверь: {}",
                        metrics.formatted_time(),
                        metrics.temperature,
                        metrics.humidity,
                        metrics.pressure,
                        if metrics.door_open {
                            "открыта"
                        } else {
                            "закрыта"
                        },
                    );

                    if metrics.door_open {
                        warn!("🚨 Обнаружена открытая дверь!");
                    }
                    if metrics.temperature > 30.0 {
                        warn!("⚠️  Высокая температура: {:.1}°C", metrics.temperature);
                    }

                    #[cfg(feature = "sqlite")]
                    {
                        debug!("SQL-запрос: {}", metrics.to_sql());
                    }
                }
                Err(_) => {
                    error!("Ошибка отправки: {}", e);
                }
            }

            debug!("Ожидание {} мс до следующей отправки", interval_ms);
            thread::sleep(Duration::from_millis(interval_ms));
        }
    }
}
