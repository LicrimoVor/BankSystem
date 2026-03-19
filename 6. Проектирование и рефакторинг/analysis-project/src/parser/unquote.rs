use super::{Parser, utils::do_unquote};

/// Парсер кавычек
#[derive(Debug, Clone)]
pub struct Unquote;
impl Parser for Unquote {
    type Dest = String;
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        do_unquote(input)
    }
}
/// Конструктор [Unquote]
pub fn unquote() -> Unquote {
    Unquote
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unquote() {
        assert_eq!(
            Unquote.parse(r#""411""#.into()),
            Ok(("".into(), "411".into()))
        );
        assert_eq!(Unquote.parse(r#" "411""#.into()), Err(()));
        assert_eq!(Unquote.parse(r#"411"#.into()), Err(()));

        assert_eq!(
            Unquote.parse(r#""ni\\c\"e""#.into()),
            Ok(("".into(), r#"ni\c"e"#.into()))
        );
    }
}
//
