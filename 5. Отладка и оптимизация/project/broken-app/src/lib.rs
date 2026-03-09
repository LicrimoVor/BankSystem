pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
///
/// FIXED
pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    unsafe {
        for idx in 0..values.len() {
            let v = *values.get_unchecked(idx);
            if v % 2 == 0 {
                acc += v;
            }
        }
    }
    acc
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
///
/// FIXED
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed);

    let mut count = 0;
    unsafe {
        let i_st = raw as *mut u8;
        for i in 0..len {
            if *i_st.add(i) != 0_u8 {
                count += 1;
            }
        }
        drop(Box::from_raw(raw));
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
///
/// FIXED
pub fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
///
/// FIXED
pub fn average_positive(values: &[i64]) -> f64 {
    let filtered: Vec<&i64> = values
        .iter()
        .filter(|arg0| arg0.is_positive() || **arg0 == 0)
        .collect();
    if filtered.is_empty() {
        return 0.0;
    }
    let len = filtered.len() as f64;
    let sum = filtered.into_iter().sum::<i64>() as f64;
    sum / len
}

// /// Use-after-free: возвращает значение после освобождения бокса.
// /// UB, проявится под ASan/Miri.
// pub unsafe fn use_after_free() -> i32 {
//     let b = Box::new(42_i32);
//     let raw = Box::into_raw(b);
//     let val = *raw;
//     drop(Box::from_raw(raw));
//     val + *raw
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_use_after_free() {
//         assert_eq!(0, unsafe { use_after_free() });
//     }
// }
