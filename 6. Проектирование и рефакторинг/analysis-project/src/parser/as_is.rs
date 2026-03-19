use super::Parser;

/// Парсер, возвращающий результат как есть
#[derive(Debug, Clone)]
struct AsIs;
impl Parser for AsIs {
    type Dest = String;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((&input[input.len()..], input.into()))
    }
}
