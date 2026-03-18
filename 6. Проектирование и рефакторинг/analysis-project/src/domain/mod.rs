// просто обёртки
// подсказка: почему бы не заменить на один дженерик?
/// Обёртка для парсинга [AssetDsc]
pub fn just_parse_asset_dsc(input: String) -> Result<(String, AssetDsc), ()> {
    <AssetDsc as Parsable>::parser().parse(input)
}
/// Обёртка для парсинга [Backet]
pub fn just_parse_backet(input: String) -> Result<(String, Backet), ()> {
    <Backet as Parsable>::parser().parse(input)
}
/// Обёртка для парсинга [UserCash]
pub fn just_user_cash(input: String) -> Result<(String, UserCash), ()> {
    <UserCash as Parsable>::parser().parse(input)
}
/// Обёртка для парсинга [UserBacket]
pub fn just_user_backet(input: String) -> Result<(String, UserBacket), ()> {
    <UserBacket as Parsable>::parser().parse(input)
}
/// Обёртка для парсинга [UserBackets]
pub fn just_user_backets(input: String) -> Result<(String, UserBackets), ()> {
    <UserBackets as Parsable>::parser().parse(input)
}
/// Обёртка для парсинга [Announcements]
pub fn just_parse_anouncements(input: String) -> Result<(String, Announcements), ()> {
    <Announcements as Parsable>::parser().parse(input)
}
