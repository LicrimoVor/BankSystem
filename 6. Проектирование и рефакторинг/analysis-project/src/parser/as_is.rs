use super::Parser;

/// Парсер, возвращающий результат как есть
#[derive(Debug, Clone)]
struct AsIs;
impl Parser for AsIs {
    type Dest = String;
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        Ok((input[input.len()..].to_string(), input.into()))
    }
}
