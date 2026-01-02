pub mod metrics;
pub mod receiver;
pub mod sender;

pub use metrics::RoomMetrics;
pub use receiver::MetricsReceiver;
pub use sender::MetricsSender;

#[cfg(feature = "logging")]
pub mod logging;

#[cfg(not(feature = "logging"))]
mod logging;

#[cfg(feature = "logging")]
pub use logging::{debug, error, info, init_logger, trace, warn};

#[cfg(not(feature = "logging"))]
pub use logging::init_logger;
