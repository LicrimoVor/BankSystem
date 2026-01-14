#[cfg(feature = "logging")]
pub use log::{debug, error, info, trace, warn};

// Макросы-заглушки, когда фича logging отключена
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

// Функция инициализации логирования
#[cfg(feature = "logging")]
pub fn init_logger() {
    env_logger::init();
    info!("Логирование инициализировано");
}

#[cfg(not(feature = "logging"))]
pub fn init_logger() {
    // Ничего не делаем, когда логирование отключено
}

pub trait Logger {
    fn log(&mut self, message: &str);
    fn new(formaters: Vec<Box<dyn Fn(String) -> String>>) -> Self
    where
        Self: Sized;
}

pub struct ConsoleLogger {
    formaters: Vec<Box<dyn Fn(String) -> String>>,
}

pub struct MemoryLogger {
    formaters: Vec<Box<dyn Fn(String) -> String>>,
    messages: Vec<String>,
}

impl Logger for ConsoleLogger {
    fn new(formaters: Vec<Box<dyn Fn(String) -> String>>) -> Self
    where
        Self: Sized,
    {
        Self { formaters }
    }

    fn log(&mut self, message: &str) {
        let message = self
            .formaters
            .iter()
            .fold(message.to_string(), |acc, func| func(acc));
        println!("{}", message);
    }
}

impl Logger for MemoryLogger {
    fn new(formaters: Vec<Box<dyn Fn(String) -> String>>) -> Self
    where
        Self: Sized,
    {
        Self {
            formaters,
            messages: vec![],
        }
    }

    fn log(&mut self, message: &str) {
        let message = self
            .formaters
            .iter()
            .fold(message.to_string(), |acc, func| func(acc));
        self.messages.push(message);
    }
}
