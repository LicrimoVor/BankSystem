#[macro_export(local_inner_macros)]
macro_rules! logging {
    (info, ($($arg:tt)+)) => {
        #[cfg(feature = "logging")]
        log::info!($($arg)+)
    };

    (warn, ($($arg:tt)+)) => {
        #[cfg(feature = "logging")]
        log::warn!($($arg)+)
    };

    (error, ($($arg:tt)+)) => {
        #[cfg(feature = "logging")]
        log::error!($($arg)+)
    };

    (debug, ($($arg:tt)+)) => {
        #[cfg(feature = "logging")]
        log::debug!($($arg)+)
    };
}
