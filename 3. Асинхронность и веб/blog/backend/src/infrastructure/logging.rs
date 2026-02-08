use tracing::info;

/// Инициализация логирования для всего приложения
pub fn logging_init() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
    info!("Logging initialized");
}
