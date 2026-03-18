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
