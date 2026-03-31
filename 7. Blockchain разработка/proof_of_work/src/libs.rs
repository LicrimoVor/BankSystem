// Вспомогательная функция для проверки уровня сложности
pub fn leading_zeros(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .take_while(|&&b| b == 0)
        .count()
        .try_into()
        .unwrap()
}
