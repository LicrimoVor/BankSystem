use crate::RoomMetrics;
use crate::{debug, error, info, trace, warn};
use bincode;
use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

pub struct MetricsReceiver {
    socket: UdpSocket,
}

impl MetricsReceiver {
    pub fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(bind_addr)?;
        info!("Ресивер запущен на {}", bind_addr);
        Ok(Self { socket })
    }

    // НОВЫЙ МЕТОД: запускает приём в отдельном потоке и возвращает канал для получения данных
    pub fn start_with_channel(
        self,
    ) -> (
        thread::JoinHandle<()>,
        mpsc::Receiver<(RoomMetrics, std::net::SocketAddr)>,
    ) {
        let (tx, rx) = mpsc::channel();

        info!("Запуск приёмника в отдельном потоке с каналом");

        let handle = thread::spawn(move || {
            if let Err(e) = self.receive_loop_with_channel(tx) {
                error!("Ошибка в receive_loop_with_channel: {}", e);
            }
        });

        (handle, rx)
    }

    // Цикл приёма с отправкой в канал
    fn receive_loop_with_channel(
        self,
        tx: mpsc::Sender<(RoomMetrics, std::net::SocketAddr)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = [0u8; 1024];

        info!("Канал приёма данных активирован");

        loop {
            debug!("Ожидание данных...");
            match self.socket.recv_from(&mut buf) {
                Ok((size, src_addr)) => match bincode::deserialize::<RoomMetrics>(&buf[..size]) {
                    Ok(metrics) => {
                        debug!("Успешная десериализация #{:?}", metrics);

                        if metrics.door_open {
                            warn!("🚨 Получены данные с открытой дверью от {}", src_addr);
                        }

                        if tx.send((metrics, src_addr)).is_err() {
                            error!("Канал закрыт, завершение потока приёма");
                            break;
                        }

                        trace!("Метрики отправлены в канал");
                    }
                    Err(e) => {
                        error!("Ошибка десериализации: {}", e);
                        debug!("Сырые данные: {:?}", &buf[..size]);
                    }
                },
                Err(e) => {
                    error!("Ошибка получения данных: {}", e);
                }
            }
        }

        Ok(())
    }
}
