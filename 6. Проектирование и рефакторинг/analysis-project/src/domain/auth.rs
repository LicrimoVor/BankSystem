const AUTHDATA_SIZE: usize = 1024;

// подсказка: довольно много места на стэке
/// Данные для авторизации
#[derive(Debug, Clone, PartialEq)]
pub struct AuthData([u8; AUTHDATA_SIZE]);
impl Parsable for AuthData {
    type Parser = Map<Take<stdp::Byte>, fn(Vec<u8>) -> Self>;
    fn parser() -> Self::Parser {
        map(take(AUTHDATA_SIZE, stdp::Byte), |authdata| {
            AuthData(authdata.try_into().unwrap_or([0; AUTHDATA_SIZE]))
        })
    }
}
