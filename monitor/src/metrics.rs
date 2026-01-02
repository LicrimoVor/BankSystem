use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMetrics {
    pub timestamp: u64,   // Unix timestamp в секундах
    pub temperature: f32, // °C
    pub humidity: f32,    // %
    pub pressure: f32,    // hPa
    pub noise_level: f32,
    pub door_open: bool,
}

impl RoomMetrics {
    pub fn new(
        temperature: f32,
        humidity: f32,
        pressure: f32,
        noise_level: f32,
        door_open: bool,
    ) -> Self {
        Self {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            temperature,
            humidity,
            pressure,
            noise_level,
            door_open,
        }
    }

    #[cfg(feature = "random")]
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        Self::new(
            rng.random_range(18.0..25.0),
            rng.random_range(30.0..60.0),
            rng.random_range(980.0..1020.0),
            rng.random_range(0.0..100.0),
            rng.random_bool(0.1), // 10% chance door is open
        )
    }

    // Альтернативная реализация без фичи random
    #[cfg(not(feature = "random"))]
    pub fn random() -> Self {
        // Простая детерминистическая реализация
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let hash = hasher.finish();

        Self::new(
            20.0 + ((hash % 1000) as f32 / 100.0),  // 20.0-30.0
            40.0 + ((hash % 1000) as f32 / 50.0),   // 40.0-60.0
            1000.0 + ((hash % 400) as f32 - 200.0), // 800.0-1200.0
            50.0 + ((hash % 100) as f32),
            (hash % 10) == 0, // 10% chance
        )
    }

    // Дополнительный метод, доступный только с фичей "sqlite"
    #[cfg(feature = "sqlite")]
    pub fn to_sql(&self) -> String {
        format!(
            "INSERT INTO metrics (timestamp, temperature, humidity, pressure, door_open) VALUES ({}, {:.1}, {:.1}, {:.1}, {})",
            self.timestamp, self.temperature, self.humidity, self.pressure, self.door_open
        )
    }

    // Метод для форматированного отображения времени
    pub fn formatted_time(&self) -> String {
        format!("{}s", self.timestamp)
    }
}
