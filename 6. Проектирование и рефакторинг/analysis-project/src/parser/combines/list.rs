use super::super::Parser;

/// Комбинатор списка из любого числа элементов, которые надо читать
/// вложенным парсером. Граница списка определяется квадратными (`[`&`]`)
/// скобками.
/// Для простоты реализации, после каждого элемента списка должна быть запятая
#[derive(Debug, Clone)]
pub struct List<T> {
    parser: T,
}
impl<T: Parser> Parser for List<T> {
    type Dest = Vec<T::Dest>;
    fn parse(&self, input: String) -> Result<(String, Self::Dest), ()> {
        let mut remaining = input
            .trim_start()
            .strip_prefix('[')
            .ok_or(())?
            .trim_start()
            .to_string();
        let mut result = Vec::new();
        while !remaining.is_empty() {
            match remaining.strip_prefix(']') {
                Some(remaining) => return Ok((remaining.trim_start().to_string(), result)),
                None => {
                    let (new_remaining, item) = self.parser.parse(remaining.to_string())?;
                    let new_remaining = new_remaining
                        .trim_start()
                        .strip_prefix(',')
                        .ok_or(())?
                        .trim_start()
                        .to_string();
                    result.push(item);
                    remaining = new_remaining;
                }
            }
        }
        Err(()) // строка кончилась, не закрыв скобку
    }
}
/// Конструктор для [List]
pub fn list<T: Parser>(parser: T) -> List<T> {
    List { parser }
}

#[cfg(test)]
mod test {
    use super::super::super::stdp;
    use super::*;

    #[test]
    fn test_list() {
        assert_eq!(
            list(stdp::U32).parse("[1,2,3,4,]".into()),
            Ok(("".into(), vec![1, 2, 3, 4,]))
        );
        assert_eq!(
            list(stdp::U32).parse(" [ 1 , 2 , 3 , 4 , ] nice".into()),
            Ok(("nice".into(), vec![1, 2, 3, 4,]))
        );
        assert_eq!(list(stdp::U32).parse("1,2,3,4,".into()), Err(()));
        assert_eq!(list(stdp::U32).parse("[]".into()), Ok(("".into(), vec![])));
    }
}
