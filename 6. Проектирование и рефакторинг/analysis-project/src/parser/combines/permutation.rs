use super::super::Parser;

/// Комбинатор, который возвращает результаты дочерних парсеров, если их
/// удалось применить друг после друга в любом порядке. Результат возвращается в
/// том порядке, в каком `Permutation` был сконструирован
/// (аналог `permutation` из `nom`)
#[derive(Debug, Clone)]
pub struct Permutation<T> {
    parsers: T,
}
impl<A0, A1> Parser for Permutation<(A0, A1)>
where
    A0: Parser,
    A1: Parser,
{
    type Dest = (A0::Dest, A1::Dest);
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        match self.parsers.0.parse(input.clone()) {
            Ok((remaining, a0)) => self
                .parsers
                .1
                .parse(remaining)
                .map(|(remaining, a1)| (remaining, (a0, a1))),
            Err(()) => self.parsers.1.parse(input).and_then(|(remaining, a1)| {
                self.parsers
                    .0
                    .parse(remaining)
                    .map(|(remaining, a0)| (remaining, (a0, a1)))
            }),
        }
    }
}
/// Конструктор [Permutation] для двух парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn permutation2<A0: Parser, A1: Parser>(a0: A0, a1: A1) -> Permutation<(A0, A1)> {
    Permutation { parsers: (a0, a1) }
}
impl<A0, A1, A2> Parser for Permutation<(A0, A1, A2)>
where
    A0: Parser,
    A1: Parser,
    A2: Parser,
{
    type Dest = (A0::Dest, A1::Dest, A2::Dest);
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        match self.parsers.0.parse(input.clone()) {
            Ok((remaining, a0)) => match self.parsers.1.parse(remaining.clone()) {
                Ok((remaining, a1)) => self
                    .parsers
                    .2
                    .parse(remaining)
                    .map(|(remaining, a2)| (remaining, (a0, a1, a2))),
                Err(()) => self
                    .parsers
                    .2
                    .parse(remaining.clone())
                    .and_then(|(remaining, a2)| {
                        self.parsers
                            .1
                            .parse(remaining)
                            .map(|(remaining, a1)| (remaining, (a0, a1, a2)))
                    }),
            },
            Err(()) => {
                match self.parsers.1.parse(input.clone()) {
                    Ok((remaining, a1)) => {
                        match self.parsers.0.parse(remaining.clone()) {
                            Ok((remaining, a0)) => self
                                .parsers
                                .2
                                .parse(remaining)
                                .map(|(remaining, a2)| (remaining, (a0, a1, a2))),
                            Err(()) => self.parsers.2.parse(remaining.clone()).and_then(
                                |(remaining, a2)| {
                                    self.parsers
                                        .0
                                        .parse(remaining)
                                        .map(|(remaining, a0)| (remaining, (a0, a1, a2)))
                                },
                            ),
                        }
                    }
                    Err(()) => self
                        .parsers
                        .2
                        .parse(input.clone())
                        .and_then(|(remaining, a2)| {
                            match self.parsers.0.parse(remaining.clone()) {
                                Ok((remaining, a0)) => self
                                    .parsers
                                    .1
                                    .parse(remaining)
                                    .map(|(remaining, a1)| (remaining, (a0, a1, a2))),
                                Err(()) => self.parsers.1.parse(remaining.clone()).and_then(
                                    |(remaining, a1)| {
                                        self.parsers
                                            .0
                                            .parse(remaining)
                                            .map(|(remaining, a0)| (remaining, (a0, a1, a2)))
                                    },
                                ),
                            }
                        }),
                }
            }
        }
    }
}
/// Конструктор [Permutation] для трёх парсеров
/// (в Rust нет чего-то, вроде variadic templates из C++)
pub fn permutation3<A0: Parser, A1: Parser, A2: Parser>(
    a0: A0,
    a1: A1,
    a2: A2,
) -> Permutation<(A0, A1, A2)> {
    Permutation {
        parsers: (a0, a1, a2),
    }
}
