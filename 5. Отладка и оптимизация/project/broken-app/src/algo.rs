use fib_rs::Fib;
use std::collections::HashMap;

/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            // лишняя копия, хотя можно было пушить значение напрямую
            out.push(*v);
            out.sort_unstable(); // бесполезная сортировка на каждой вставке
        }
    }
    out
}

/// Бестрая реализация реализация.
pub fn fast_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        if !out.contains(v) {
            out.push(*v);
        }
    }
    out.sort_unstable();
    out
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

/// Быстрая версия фибоначи с мемоизацией.
pub fn fast_fib(n: u64) -> u64 {
    let mut memo = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);

    match n {
        0 => 0,
        1 => 1,
        _ => fib(n, &mut memo),
    }
}

fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    match memo.get(&n) {
        Some(res) => res.clone(),
        None => {
            let res = fib(n - 1, memo) + fib(n - 2, memo);
            memo.insert(n, res);
            res
        }
    }
}

pub fn lib_fib(n: u64) -> u64 {
    Fib::single(n as u128).to_u64_digits()[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedup_preserves_uniques() {
        let uniq = slow_dedup(&[5, 5, 1, 2, 2, 3]);
        assert_eq!(uniq, vec![1, 2, 3, 5]);
    }

    #[test]
    fn slow_fib_small_numbers() {
        assert_eq!(slow_fib(10), 55);
    }

    #[test]
    fn fast_fib_small_numbers() {
        assert_eq!(fast_fib(10), 55);
    }

    #[test]
    fn lib_fib_small_numbers() {
        assert_eq!(lib_fib(10), 55);
    }
}
