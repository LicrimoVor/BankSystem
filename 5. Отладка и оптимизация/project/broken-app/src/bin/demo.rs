use broken_app::{algo, concurrency::race_increment, leak_buffer, normalize, sum_even};

fn main() {
    let nums = [1, 2, 3, 4];
    println!("sum_even: {}", sum_even(&nums));

    let data = [1_u8, 0, 2, 3];
    println!("non-zero bytes: {}", leak_buffer(&data));

    let text = " Hello World ";
    println!("normalize: {}", normalize(text));

    let fib = algo::slow_fib(30);
    println!("fib(30): {}", fib);

    let uniq = algo::slow_dedup(&[
        1, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4,
        4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4, 4, 2, 2, 3, 1, 4,
    ]);
    println!("dedup: {:?}", uniq);

    let race = race_increment(1000, 100);
    println!("race: {}", race);
}
