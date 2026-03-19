use super::{Parser, utils::do_unquote_non_escaped};

/// Парсер константных строк
/// (аналог `nom::bytes::complete::tag`)
#[derive(Debug, Clone)]
pub struct Tag {
    tag: &'static str,
}
impl Parser for Tag {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        Ok((input.strip_prefix(self.tag).ok_or(())?, ()))
    }
}
/// Конструктор [Tag]
pub fn tag(tag: &'static str) -> Tag {
    Tag { tag }
}

/// Парсер [тэга](Tag), обёрнутого в кавычки
#[derive(Debug, Clone)]
pub struct QuotedTag(Tag);
impl Parser for QuotedTag {
    type Dest = ();
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, candidate) = do_unquote_non_escaped(input)?;
        if !self.0.parse(candidate)?.0.is_empty() {
            return Err(());
        }
        Ok((remaining, ()))
    }
}
/// Конструктор [QuotedTag]
pub fn quoted_tag(tag: &'static str) -> QuotedTag {
    QuotedTag(Tag { tag })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag() {
        assert_eq!(
            tag("key=").parse("key=value".into()),
            Ok(("value".into(), ()))
        );
        assert_eq!(tag("key=").parse("key:value".into()), Err(()));
    }

    #[test]
    fn test_quoted_tag() {
        assert_eq!(
            quoted_tag("key").parse(r#""key"=value"#.into()),
            Ok(("=value".into(), ()))
        );
        assert_eq!(quoted_tag("key").parse(r#""key:"value"#.into()), Err(()));
        assert_eq!(quoted_tag("key").parse(r#"key=value"#.into()), Err(()));
    }
}
