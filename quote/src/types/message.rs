use super::stock::{StockQuote, Ticker};
use bincode::error::EncodeError;
use serde::{Deserialize, Serialize};

/// Сообщение сервера
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UdpMessage {
    Init(Vec<StockQuote>),
    Stock(StockQuote),
    Close(Ticker),
    Pong,
    Ping,
    Disconnect,
}

/// Формат сообщения
pub enum UdpMessageFormat {
    Bin,
    Json,
    Text,
}

impl UdpMessage {
    pub fn to_format(&self, format: &UdpMessageFormat) -> Result<Vec<u8>, EncodeError> {
        match format {
            UdpMessageFormat::Bin => self.to_bin(),
            UdpMessageFormat::Json => self.to_json(),
            UdpMessageFormat::Text => Ok(self.to_string().as_bytes().to_vec()),
        }
    }

    pub fn to_bin(&self) -> Result<Vec<u8>, EncodeError> {
        let config = bincode::config::standard();
        match self {
            UdpMessage::Init(stocks) => {
                let mut mess = "INIT".as_bytes().to_vec();

                let Ok(mut stocks) = bincode::encode_to_vec(stocks, config) else {
                    return Err(EncodeError::Other("Ошибка кодирования"));
                };
                mess.append(&mut stocks);
                Ok(mess)
            }
            UdpMessage::Stock(stock) => {
                let mut mess = "STOC".as_bytes().to_vec();
                let Ok(mut stock) = bincode::encode_to_vec(stock, config) else {
                    return Err(EncodeError::Other("Ошибка кодирования"));
                };
                mess.append(&mut stock);
                Ok(mess)
            }
            UdpMessage::Close(ticker) => {
                let mut mess = "CLOS".as_bytes().to_vec();
                mess.append(&mut ticker.as_bytes().to_vec());
                Ok(mess)
            }
            UdpMessage::Pong => Ok("PONG".as_bytes().to_vec()),
            UdpMessage::Ping => Ok("PING".as_bytes().to_vec()),
            UdpMessage::Disconnect => Ok("DISC".as_bytes().to_vec()),
        }
    }

    pub fn to_json(&self) -> Result<Vec<u8>, EncodeError> {
        // уверен что таким образом можно сделать все другие методы,
        // но я что зря делал match?)
        let Ok(res) = serde_json::to_vec(&self) else {
            return Err(EncodeError::Other("Ошибка кодирования"));
        };
        return Ok(res);
    }

    pub fn to_string(&self) -> String {
        match self {
            UdpMessage::Init(stocks) => {
                let mut mess = "Welcome! Last stocks (ticker|price|volume|timestamp):".to_string();
                for stock in stocks {
                    mess.push_str(format!("\n{}", stock.to_string()).as_str());
                }
                mess
            }
            UdpMessage::Stock(stock) => stock.to_string(),
            UdpMessage::Close(ticker) => format!("CLOSE STOCK: {}", ticker),
            UdpMessage::Pong => "PONG".to_string(),
            UdpMessage::Ping => "Ping".to_string(),
            UdpMessage::Disconnect => "DISCONNECT".to_string(),
        }
    }
}

impl UdpMessage {
    pub fn from_format(data: &[u8], format: &UdpMessageFormat) -> Result<UdpMessage, EncodeError> {
        match format {
            UdpMessageFormat::Bin => UdpMessage::from_bin(data),
            UdpMessageFormat::Json => UdpMessage::from_json(data),
            UdpMessageFormat::Text => UdpMessage::from_string(data),
        }
    }

    pub fn from_json(data: &[u8]) -> Result<UdpMessage, EncodeError> {
        let Ok(mes) = serde_json::from_slice(data) else {
            return Err(EncodeError::Other("Ошибка кодирования"));
        };
        Ok(mes)
    }

    pub fn from_string(data: &[u8]) -> Result<UdpMessage, EncodeError> {
        let Ok(data) = String::from_utf8(data.to_vec()) else {
            return Err(EncodeError::Other("Ошибка кодирования"));
        };
        let mes = &data[..4];

        match mes {
            "Welc" => {
                let mut stocks = vec![];
                for line in data.lines().skip(1) {
                    let Some(stock) = StockQuote::from_string(line) else {
                        return Err(EncodeError::Other("Ошибка кодирования"));
                    };
                    stocks.push(stock);
                }

                Ok(UdpMessage::Init(stocks))
            }
            "CLOS" => Ok(UdpMessage::Close(data[13..].to_string())),
            "PONG" => Ok(UdpMessage::Pong),
            "PING" => Ok(UdpMessage::Ping),
            "DISC" => Ok(UdpMessage::Disconnect),
            _ => StockQuote::from_string(&data)
                .ok_or(EncodeError::Other("Ошибка кодирования"))
                .map(UdpMessage::Stock),
        }
    }

    pub fn from_bin(data: &[u8]) -> Result<UdpMessage, EncodeError> {
        let config = bincode::config::standard();

        let Ok(mes_type) = String::from_utf8(data[..4].to_vec()) else {
            return Err(EncodeError::Other("Ошибка кодирования"));
        };

        match mes_type.as_str() {
            "INIT" => {
                let Ok((stocks, _)) = bincode::decode_from_slice(&data[4..], config) else {
                    return Err(EncodeError::Other("Ошибка кодирования"));
                };
                Ok(UdpMessage::Init(stocks))
            }
            "STOC" => {
                let Ok((stock, _)) = bincode::decode_from_slice(&data[4..], config) else {
                    return Err(EncodeError::Other("Ошибка кодирования"));
                };
                Ok(UdpMessage::Stock(stock))
            }
            "CLOS" => Ok(UdpMessage::Close(
                String::from_utf8(data[4..].to_vec()).unwrap(),
            )),
            "PONG" => Ok(UdpMessage::Pong),
            "PING" => Ok(UdpMessage::Ping),
            "DISC" => Ok(UdpMessage::Disconnect),
            _ => Err(EncodeError::Other("Неизвестный тип сообщения")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_formats(mess: UdpMessage) {
        let bin = mess.to_bin().unwrap();
        let json = mess.to_json().unwrap();
        let str = mess.to_string();

        let json_from_str = UdpMessage::from_json(json.as_slice()).unwrap();
        let bin_from_str = UdpMessage::from_bin(bin.as_slice()).unwrap();
        let str_from_str = UdpMessage::from_string(str.as_bytes()).unwrap();

        assert_eq!(mess, json_from_str);
        assert_eq!(mess, bin_from_str);
        assert_eq!(mess, str_from_str);
    }

    #[test]
    fn test_init_empty() {
        let init = UdpMessage::Init(vec![]);
        check_formats(init);
    }

    #[test]
    fn test_init_full() {
        let init = UdpMessage::Init(vec![
            StockQuote::one(),
            StockQuote::two(),
            StockQuote::one(),
        ]);
        check_formats(init);
    }

    #[test]
    fn test_stock() {
        let stock = UdpMessage::Stock(StockQuote::one());
        check_formats(stock);
    }

    #[test]
    fn test_close() {
        let close = UdpMessage::Close("BCD".to_string());
        check_formats(close);
    }

    #[test]
    fn test_pong() {
        let pong = UdpMessage::Pong;
        check_formats(pong);
    }

    #[test]
    fn test_disconnect() {
        let disconnect = UdpMessage::Disconnect;
        check_formats(disconnect);
    }
}
