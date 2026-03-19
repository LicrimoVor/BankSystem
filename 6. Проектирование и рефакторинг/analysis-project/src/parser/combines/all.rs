use super::super::Parser;

/// Комбинатор, который требует, чтобы все дочерние парсеры отработали,
/// (аналог `all` из `nom`)
#[derive(Debug, Clone)]
pub struct All<T> {
    parser: T,
}
impl<A0, A1> Parser for All<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    type Dest = (A0::Dest, A1::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        self.parser
            .1
            .parse(remaining)
            .map(|(remaining, a1)| (remaining, (a0, a1)))
    }
}
/// Конструктор [All] для двух парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn all2<A0: Parser, A1: Parser>(a0: A0, a1: A1) -> All<(A0, A1)> {
    All { parser: (a0, a1) }
}
impl<A0, A1, A2> Parser for All<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        let (remaining, a1) = self.parser.1.parse(remaining)?;
        self.parser
            .2
            .parse(remaining)
            .map(|(remaining, a2)| (remaining, (a0, a1, a2)))
    }
}
/// Конструктор [All] для трёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn all3<A0: Parser, A1: Parser, A2: Parser>(a0: A0, a1: A1, a2: A2) -> All<(A0, A1, A2)> {
    All {
        parser: (a0, a1, a2),
    }
}
impl<A0, A1, A2, A3> Parser for All<(A0, A1, A2, A3)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
    A3: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest, A3::Dest);
    fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, Self::Dest), ()> {
        let (remaining, a0) = self.parser.0.parse(input)?;
        let (remaining, a1) = self.parser.1.parse(remaining)?;
        let (remaining, a2) = self.parser.2.parse(remaining)?;
        self.parser
            .3
            .parse(remaining)
            .map(|(remaining, a3)| (remaining, (a0, a1, a2, a3)))
    }
}
/// Конструктор [All] для четырёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn all4<A0: Parser, A1: Parser, A2: Parser, A3: Parser>(
    a0: A0,
    a1: A1,
    a2: A2,
    a3: A3,
) -> All<(A0, A1, A2, A3)> {
    All {
        parser: (a0, a1, a2, a3),
    }
}
