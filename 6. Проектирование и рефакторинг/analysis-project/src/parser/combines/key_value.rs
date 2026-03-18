use super::super::{
    Parser,
    tag::{QuotedTag, Tag, quoted_tag, tag},
};
use super::{
    all::{All, all2},
    delimited::{Delimited, delimited},
    strip_whitespace::{StripWhitespace, strip_whitespace},
};

/// Комбинатор, который вытаскивает значения из пары `"ключ":значение,`.
/// Для простоты реализации, запятая всегда нужна в конце пары ключ-значение,
/// простое '"ключ":значение' читаться не будет
#[derive(Debug, Clone)]
struct KeyValue<T> {
    parser: Delimited<
        All<(StripWhitespace<QuotedTag>, StripWhitespace<Tag>)>,
        StripWhitespace<T>,
        StripWhitespace<Tag>,
    >,
}
impl<T> Parser for KeyValue<T>
where
    T: Parser,
{
    type Dest = T::Dest;
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        self.parser.parse(input)
    }
}
/// Конструктор [KeyValue]
fn key_value<T: Parser>(key: &'static str, value_parser: T) -> KeyValue<T> {
    KeyValue {
        parser: delimited(
            all2(
                strip_whitespace(quoted_tag(key)),
                strip_whitespace(tag(":")),
            ),
            strip_whitespace(value_parser),
            strip_whitespace(tag(",")),
        ),
    }
}

#[cfg(test)]
mod test {
    use super::super::super::stdp;
    use super::*;

    #[test]
    fn test_key_value() {
        assert_eq!(
            key_value("key", stdp::U32).parse(r#""key":32,"#.into()),
            Ok(("".into(), 32))
        );
        assert_eq!(
            key_value("key", stdp::U32).parse(r#"key:32,"#.into()),
            Err(())
        );
        assert_eq!(
            key_value("key", stdp::U32).parse(r#""key":32"#.into()),
            Err(())
        );
        assert_eq!(
            key_value("key", stdp::U32).parse(r#" "key" : 32 , nice"#.into()),
            Ok(("nice".into(), 32))
        );
    }
}
