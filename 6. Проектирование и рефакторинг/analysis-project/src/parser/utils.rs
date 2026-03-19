/// Обернуть строку в кавычки, экранировав кавычки, которые в строке уже есть
pub(super) fn quote(input: &str) -> String {
    let mut result = String::from("\"");
    result.extend(
        input
            .chars()
            .map(|c| match c {
                '\\' | '"' => ['\\', c].into_iter().take(2),
                _ => [c, ' '].into_iter().take(1),
            })
            .flatten(),
    );
    result.push('"');
    result
}

/// Распарсить строку, которую ранее [обернули в кавычки](quote)
// `"abc\"def\\ghi"nice` -> (`abcd"def\ghi`, `nice`)
pub(super) fn do_unquote(input: &str) -> Result<(&str, String), ()> {
    let mut result = String::new();
    let mut escaped_now = false;
    let mut chars = input.strip_prefix("\"").ok_or(())?.chars();
    while let Some(c) = chars.next() {
        match (c, escaped_now) {
            ('"' | '\\', true) => {
                result.push(c);
                escaped_now = false;
            }
            ('\\', false) => escaped_now = true,
            ('"', false) => return Ok((chars.as_str(), result)),
            (c, _) => {
                result.push(c);
                escaped_now = false;
            }
        }
    }
    Err(()) // строка кончилась, не закрыв кавычку
}

/// Распарсить строку, обёрную в кавычки
/// (сокращённая версия [do_unquote], в которой вложенные кавычки не предусмотрены)
pub(super) fn do_unquote_non_escaped(input: &str) -> Result<(&str, &str), ()> {
    let input = input.strip_prefix("\"").ok_or(())?;
    let quote_byteidx = input.find('"').ok_or(())?;
    if 0 == quote_byteidx || Some("\\") == input.get(quote_byteidx - 1..quote_byteidx) {
        return Err(());
    }
    Ok((&input[1 + quote_byteidx..], &input[..quote_byteidx]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quote() {
        assert_eq!(quote(r#"411"#), r#""411""#.to_string());
        assert_eq!(quote(r#"4\11""#), r#""4\\11\"""#.to_string());
    }

    #[test]
    fn test_do_unquote_non_escaped() {
        assert_eq!(
            do_unquote_non_escaped(r#""411""#.into()),
            Ok(("".into(), "411".into()))
        );
        assert_eq!(do_unquote_non_escaped(r#" "411""#.into()), Err(()));
        assert_eq!(do_unquote_non_escaped(r#"411"#.into()), Err(()));
    }
}
//
