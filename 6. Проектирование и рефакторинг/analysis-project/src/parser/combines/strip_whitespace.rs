use super::super::Parser;

/// Комбинатор, пробрасывающий строку без лидирующих пробелов
#[derive(Debug, Clone)]
pub struct StripWhitespace<T> {
    parser: T,
}
impl<T: Parser> Parser for StripWhitespace<T> {
    type Dest = T::Dest;
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        self.parser
            .parse(input.trim_start().to_string())
            .map(|(remaining, parsed)| (remaining.trim_start().to_string(), parsed))
    }
}
/// Конструктор [StripWhitespace]
pub fn strip_whitespace<T: Parser>(parser: T) -> StripWhitespace<T> {
    StripWhitespace { parser }
}

#[cfg(test)]
mod test {
    use super::super::super::{stdp, tag::tag};
    use super::*;

    #[test]
    fn test_strip_whitespace() {
        assert_eq!(
            strip_whitespace(tag("hello")).parse(" hello world".into()),
            Ok(("world".into(), ()))
        );
        assert_eq!(
            strip_whitespace(tag("hello")).parse("hello".into()),
            Ok(("".into(), ()))
        );
        assert_eq!(
            strip_whitespace(stdp::U32).parse(" 42 answer".into()),
            Ok(("answer".into(), 42))
        );
    }
}
