use super::super::Parser;

/// Комбинатор для применения дочернего парсера N раз
/// (аналог `take` из `nom`)
pub struct Take<T> {
    count: usize,
    parser: T,
}
impl<T: Parser> Parser for Take<T> {
    type Dest = Vec<T::Dest>;
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        let mut remaining = input;
        let mut result = Vec::new();
        for _ in 0..self.count {
            let (new_remaining, new_result) = self.parser.parse(remaining)?;
            result.push(new_result);
            remaining = new_remaining;
        }
        Ok((remaining, result))
    }
}
/// Конструктор `Take`
pub fn take<T: Parser>(count: usize, parser: T) -> Take<T> {
    Take { count, parser }
}
