fn sum_numbers(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        sum += numbers[i];
    }
    sum
}

fn process_data(data: Vec<i32>) -> i32 {
    let result = sum_numbers(&data);
    result * 2 // намеренная ошибка: должно быть просто result
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let result = process_data(data);
    println!("Sum: {}", result);
    // Ожидается 15, но получается 30
}
