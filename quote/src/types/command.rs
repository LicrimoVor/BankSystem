use super::stock::Ticker;
use std::{net::SocketAddr, str::FromStr};

/// Команды для общения с tcp-мастером
#[derive(Debug, PartialEq)]
pub enum Command {
    /// STREAM <ip>:<port> <ticker,ticker...>
    /// создать поток
    Stream((SocketAddr, Vec<Ticker>)),

    /// STOP <ip>:<port>
    /// остановить поток
    Stop(SocketAddr),

    /// LIST
    /// список подключений
    List,

    /// DISCONNECT
    /// отключиться (завершив все потоки)
    Disconnect,

    /// TICKERS
    /// список тикеров
    Tickers,

    /// HELP
    /// выводит список команд
    Help,

    /// SHUTDOWN
    /// выключить сервер
    Shutdown(String),
}

impl Command {
    pub fn parse(s: &str) -> Result<Self, &str> {
        match s[0..4].to_uppercase().as_str() {
            "STRE" => {
                let parts: Vec<&str> = s.split(' ').collect();
                if parts.len() != 3 {
                    return Err(
                        "Неправильная команда стрима\nSTREAM <ip>:<port> <ticker,ticker...>",
                    );
                }
                let tickers: Vec<Ticker> = parts[2]
                    .split(',')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                let Ok(addr) = SocketAddr::from_str(parts[1]) else {
                    return Err(
                        "Неправильная команда стрима\nSTREAM <ip>:<port> <ticker,ticker...>",
                    );
                };

                Ok(Command::Stream((addr, tickers)))
            }
            "STOP" => {
                let parts: Vec<&str> = s.split(' ').collect();
                if parts.len() != 2 {
                    return Err("Неправильная команда стоп\nSTOP <ip>:<port>");
                }
                let Ok(addr) = SocketAddr::from_str(parts[1]) else {
                    return Err("Неправильная команда стоп\nSTOP <ip>:<port>");
                };
                Ok(Command::Stop(addr))
            }
            "LIST" => Ok(Command::Disconnect),
            "DISC" => Ok(Command::Disconnect),
            "TICK" => Ok(Command::Tickers),
            "HELP" => Ok(Command::Help),
            "SHUT" => {
                let Some((_, r)) = s.split_once(' ') else {
                    return Err("Неправильная команда SHUTDOWN\nSHUTDOWN <key>");
                };
                Ok(Command::Shutdown(r.to_string()))
            }
            _ => Err("Неизсветная команда. Отправьте HELP"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Command::Stream((addr, tickers)) => format!("STREAM {} {}", addr, tickers.join(",")),
            Command::Stop(addr) => format!("STOP {}", addr),
            Command::Disconnect => "DISCONNECT".to_string(),
            Command::List => "LIST".to_string(),
            Command::Tickers => "TICKERS".to_string(),
            Command::Help => "HELP".to_string(),
            Command::Shutdown(key) => format!("SHUTDOWN {}", key),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command() {
        let str_command = "STREAM 127.0.0.1:8080 BTC,ETH";
        let command_parsed = Command::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }

    #[test]
    fn test_command_stop() {
        let str_command = "STOP 127.0.0.1:8080";
        let command_parsed = Command::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }

    #[test]
    fn test_command_help() {
        let str_command = "HELP";
        let command_parsed = Command::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }

    #[test]
    fn test_command_shutdown_key() {
        let str_command = "SHUTDOWN key";
        let command_parsed = Command::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }
}
