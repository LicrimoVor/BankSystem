fn main() {
    let items = vec![
        String::from("item1"),
        String::from("item2"),
        String::from("item3"),
    ];

    process_items(&items);
}

fn process_items(items: &Vec<String>) {
    for item in items {
        process_item(item.as_str());
    }
}

fn process_item(item: &str) {
    println!("Processing: {}", item);
}

fn find_positive_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|a| **a > 0).copied().collect()
}

use std::cell::RefCell;
use std::rc::Rc;

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}

// Проблема: паника в библиотечной функции
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

use std::thread;

fn bad_example() {
    let data = vec![1, 2, 3];
    thread::spawn(move || {
        for i in data {
            println!("{i}");
        }
    })
    .join()
    .unwrap();
}

fn process_nested(data: Option<Option<i32>>) {
    match data {
        Some(Some(v)) => println!("Value: {}", v),
        Some(None) => println!("Inner is None"),
        None => println!("Outer is None"),
    }
}
