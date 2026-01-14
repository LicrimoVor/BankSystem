use super::stock::Ticker;
use std::{net::SocketAddr, str::FromStr};

/// Команды для общения с tcp-мастером
#[derive(Debug, PartialEq)]
pub enum TcpCommand {
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

impl TcpCommand {
    pub fn parse(s: &str) -> Result<Self, &str> {
        let s = s.trim();
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

                Ok(TcpCommand::Stream((addr, tickers)))
            }
            "STOP" => {
                let parts: Vec<&str> = s.split(' ').collect();
                if parts.len() != 2 {
                    return Err("Неправильная команда стоп\nSTOP <ip>:<port>");
                }
                let Ok(addr) = SocketAddr::from_str(parts[1]) else {
                    return Err("Неправильная команда стоп\nSTOP <ip>:<port>");
                };
                Ok(TcpCommand::Stop(addr))
            }
            "LIST" => Ok(TcpCommand::Disconnect),
            "DISC" => Ok(TcpCommand::Disconnect),
            "TICK" => Ok(TcpCommand::Tickers),
            "HELP" => Ok(TcpCommand::Help),
            "SHUT" => {
                let Some((_, r)) = s.split_once(' ') else {
                    return Err("Неправильная команда SHUTDOWN\nSHUTDOWN <key>");
                };
                Ok(TcpCommand::Shutdown(r.to_string()))
            }
            _ => Err("Неизсветная команда. Отправьте HELP"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TcpCommand::Stream((addr, tickers)) => {
                format!("STREAM {} {}\n", addr, tickers.join(","))
            }
            TcpCommand::Stop(addr) => format!("STOP {}\n", addr),
            TcpCommand::Disconnect => "DISCONNECT\n".to_string(),
            TcpCommand::List => "LIST\n".to_string(),
            TcpCommand::Tickers => "TICKERS\n".to_string(),
            TcpCommand::Help => "HELP\n".to_string(),
            TcpCommand::Shutdown(key) => format!("SHUTDOWN {}\n", key),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_stream_parse() {
        let str_command = "STREAM 127.0.0.1:7879 T1,T9\n";
        let command_parsed = TcpCommand::parse(str_command);
        assert!(command_parsed.is_ok());
        let parsed_command = TcpCommand::Stream((
            "127.0.0.1:7879".parse().unwrap(),
            vec!["T1".to_string(), "T9".to_string()],
        ));
        assert_eq!(parsed_command, command_parsed.unwrap());
    }

    #[test]
    fn test_command_stream() {
        let str_command = "STREAM 127.0.0.1:8080 BTC,ETH\n";
        let command_parsed = TcpCommand::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }

    #[test]
    fn test_command_stop() {
        let str_command = "STOP 127.0.0.1:8080\n";
        let command_parsed = TcpCommand::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }

    #[test]
    fn test_command_help() {
        let str_command = "HELP\n";
        let command_parsed = TcpCommand::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }

    #[test]
    fn test_command_shutdown_key() {
        let str_command = "SHUTDOWN key\n";
        let command_parsed = TcpCommand::parse(str_command);
        assert!(command_parsed.is_ok());
        let command_str = command_parsed.unwrap().to_string();
        assert_eq!(str_command, command_str);
    }
}
