use super::user::UserBackets;
use crate::parser::prelude::*;

/// Пара 'сокращённое название предмета' - 'его описание'
#[derive(Debug, Clone, PartialEq)]
pub struct AssetDsc {
    // `dsc` aka `description`
    pub id: String,
    pub dsc: String,
}
impl Parsable for AssetDsc {
    type Parser = Map<
        Delimited<
            All<(StripWhitespace<Tag>, StripWhitespace<Tag>)>,
            Permutation<(KeyValue<Unquote>, KeyValue<Unquote>)>,
            StripWhitespace<Tag>,
        >,
        fn((String, String)) -> Self,
    >;
    fn parser() -> Self::Parser {
        // комбинаторы парсеров - это круто

        // Вроде круто, но как такое придумывать самому :|
        map(
            delimited(
                all2(
                    strip_whitespace(tag("AssetDsc")),
                    strip_whitespace(tag("{")),
                ),
                permutation2(key_value("id", unquote()), key_value("dsc", unquote())),
                strip_whitespace(tag("}")),
            ),
            |(id, dsc)| AssetDsc { id, dsc },
        )
    }
}

/// Сведение о предмете в некотором количестве
#[derive(Debug, Clone, PartialEq)]
pub struct Backet {
    pub asset_id: String,
    pub count: u32,
}
impl Parsable for Backet {
    type Parser = Map<
        Delimited<
            All<(StripWhitespace<Tag>, StripWhitespace<Tag>)>,
            Permutation<(KeyValue<Unquote>, KeyValue<stdp::U32>)>,
            StripWhitespace<Tag>,
        >,
        fn((String, u32)) -> Self,
    >;
    fn parser() -> Self::Parser {
        map(
            delimited(
                all2(strip_whitespace(tag("Backet")), strip_whitespace(tag("{"))),
                permutation2(
                    key_value("asset_id", unquote()),
                    key_value("count", stdp::U32),
                ),
                strip_whitespace(tag("}")),
            ),
            |(asset_id, count)| Backet { asset_id, count },
        )
    }
}

/// Список опубликованных бакетов
#[derive(Debug, Clone, PartialEq)]
pub struct Announcements(Vec<UserBackets>);
impl Parsable for Announcements {
    type Parser = Map<List<<UserBackets as Parsable>::Parser>, fn(Vec<UserBackets>) -> Self>;
    fn parser() -> Self::Parser {
        fn from_vec(vec: Vec<UserBackets>) -> Announcements {
            Announcements(vec)
        }
        map(list(UserBackets::parser()), from_vec)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asset_dsc() {
        assert_eq!(
            all2(
                strip_whitespace(tag("AssetDsc")),
                strip_whitespace(tag("{"))
            )
            .parse(" AssetDsc { ".into()),
            Ok(("".into(), ((), ())))
        );

        assert_eq!(
            AssetDsc::parser().parse(r#"AssetDsc{"id":"usd","dsc":"USA dollar",}"#.into()),
            Ok((
                "".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );
        assert_eq!(
            AssetDsc::parser()
                .parse(r#" AssetDsc { "id" : "usd" , "dsc" : "USA dollar" , } "#.into()),
            Ok((
                "".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );
        assert_eq!(
            AssetDsc::parser()
                .parse(r#" AssetDsc { "id" : "usd" , "dsc" : "USA dollar" , } nice "#.into()),
            Ok((
                "nice ".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );

        assert_eq!(
            AssetDsc::parser().parse(r#"AssetDsc{"dsc":"USA dollar","id":"usd",}"#.into()),
            Ok((
                "".into(),
                AssetDsc {
                    id: "usd".into(),
                    dsc: "USA dollar".into()
                }
            ))
        );
    }

    #[test]
    fn test_backet() {
        assert_eq!(
            Backet::parser().parse(r#"Backet{"asset_id":"usd","count":42,}"#.into()),
            Ok((
                "".into(),
                Backet {
                    asset_id: "usd".into(),
                    count: 42
                }
            ))
        );
        assert_eq!(
            Backet::parser().parse(r#"Backet{"count":42,"asset_id":"usd",}"#.into()),
            Ok((
                "".into(),
                Backet {
                    asset_id: "usd".into(),
                    count: 42
                }
            ))
        );
    }
}
