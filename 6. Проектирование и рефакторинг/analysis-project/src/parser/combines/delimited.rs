use super::super::Parser;

/// Комбинатор, чтобы распарсить нужное, окружённое в начале и в конце чем-то
/// обязательным, не участвующем в результате.
/// Пробрасывает строку в парсер1, оставшуюся строку после первого
/// парсинга - в парсер2, оставшуюся строку после второго парсинга - в парсер3.
/// Результат парсера2 будет результатом этого комбинатора, а оставшейся
/// строкой - строка, оставшаяся после парсера3.
/// (аналог `delimited` из `nom`)
#[derive(Debug, Clone)]
pub struct Delimited<Prefix, T, Suffix> {
    prefix_to_ignore: Prefix,
    dest_parser: T,
    suffix_to_ignore: Suffix,
}
impl<Prefix, T, Suffix> Parser for Delimited<Prefix, T, Suffix>
where
    Prefix: Parser,
    T: Parser,
    Suffix: Parser,
{
    type Dest = T::Dest;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, _) = self.prefix_to_ignore.parse(input)?;
        let (remaining, result) = self.dest_parser.parse(remaining)?;
        self.suffix_to_ignore
            .parse(remaining)
            .map(|(remaining, _)| (remaining, result))
    }
}
/// Конструктор [Delimited]
pub fn delimited<Prefix, T, Suffix>(
    prefix_to_ignore: Prefix,
    dest_parser: T,
    suffix_to_ignore: Suffix,
) -> Delimited<Prefix, T, Suffix>
where
    Prefix: Parser,
    T: Parser,
    Suffix: Parser,
{
    Delimited {
        prefix_to_ignore,
        dest_parser,
        suffix_to_ignore,
    }
}

#[cfg(test)]
mod test {
    use super::super::super::{stdp, tag::tag};
    use super::*;

    #[test]
    fn test_delimited() {
        assert_eq!(
            delimited(tag("["), stdp::U32, tag("]")).parse("[0x32]".into()),
            Ok(("".into(), 0x32))
        );
        assert_eq!(
            delimited(tag("[".into()), stdp::U32, tag("]".into())).parse("[0x32] nice".into()),
            Ok((" nice".into(), 0x32))
        );
        assert_eq!(
            delimited(tag("[".into()), stdp::U32, tag("]")).parse("0x32]".into()),
            Err(())
        );
        assert_eq!(
            delimited(tag("[".into()), stdp::U32, tag("]")).parse("[0x32".into()),
            Err(())
        );
    }
}
