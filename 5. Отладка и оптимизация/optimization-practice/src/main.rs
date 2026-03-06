use optimization_practice::{process_json, sum_numbers};

fn main() {
    let data = (0..10000)
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let json_data = format!(r#"[{}]"#, data);
    for _ in 0..2000 {
        let numbers = process_json(&json_data.clone()).unwrap();
        let _sum = sum_numbers(&numbers);
    }
}
