pub mod as_is;
pub mod combines;
pub mod stdp;
pub mod tag;
pub mod unquote;
pub(self) mod utils;

/// Трейт, чтобы **реализовывать** и **требовать** метод 'распарсь и покажи,
/// что распарсить осталось'
trait Parser {
    type Dest;
    // подсказка: здесь можно переделать
    // на `fn parse<'a>(&self,input:&'a str)->Result<(&'a str, Self::Dest)>`
    // (возможно, самое трудоёмкое; в своих проектах проще сразу не допускать)
    fn parse<'a>(&self, input: String) -> Result<(String, Self::Dest), ()>;
}

/// Вспомогательный трейт, чтобы писать собственный десериализатор
/// (по решаемой задаче - отдалённый аналог `serde::Deserialize`)
trait Parsable: Sized {
    type Parser: Parser<Dest = Self>;
    fn parser() -> Self::Parser;
}
