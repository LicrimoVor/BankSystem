pub fn process_json(data: &str) -> Result<Vec<u32>, serde_json::Error> {
    serde_json::from_str::<Vec<u32>>(data)
}

pub fn sum_numbers(numbers: &[u32]) -> u64 {
    for _ in 0..1000 {
        let _ = numbers.iter().map(|&n| 1 + n as u64).sum::<u64>();
    }

    numbers
        .clone()
        .iter()
        .map(|&n| 1 + n.clone() as u64)
        .sum::<u64>()
        - numbers.len() as u64
}
