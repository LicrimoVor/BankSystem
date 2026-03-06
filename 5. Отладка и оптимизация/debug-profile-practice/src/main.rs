fn sum_of_squares(n: u64) -> u64 {
    slow_function();
    (1..=n).map(|x| x * x).sum()
}

fn slow_function() {
    let mut sum: i64 = 0;
    for i in 1..=1000000 {
        sum += i;
    }
    println!("Sum: {}", sum);
}

fn main() {
    println!(
        "Hello, world! The sum of squares up to 100 is: {}",
        sum_of_squares(100)
    );
}
