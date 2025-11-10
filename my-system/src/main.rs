use bank_system::{Name, Storage};
use std::env;

fn main() {
    let mut storage = Storage::new();

    // заранее добавляем пользователей
    let users = vec!["John", "Alice", "Bob", "Vasya"];
    for u in users {
        storage.add_user(u.to_string());
    }

    // собираем аргументы
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Использование:");
        eprintln!("  deposit <name> <amount>");
        eprintln!("  withdraw <name> <amount>");
        eprintln!("  balance <name>");
        return;
    }

    match args[1].as_str() {
        "deposit" => {
            if args.len() != 4 {
                eprintln!("Пример: add John 200");
                return;
            }
            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть числом");
            match storage.deposit(&name, amount.into()) {
                Ok(_) => println!("Пополнено: {} на {}", name, amount),
                Err(e) => println!("Ошибка: {}", e),
            }
        }
        "withdraw" => {
            if args.len() != 4 {
                eprintln!("Пример: withdraw John 100");
                return;
            }
            let name: Name = args[2].clone();
            let amount: i64 = args[3].parse().expect("Сумма должна быть числом");
            match storage.withdraw(&name, amount.into()) {
                Ok(_) => println!("Снято: {} на {}", name, amount),
                Err(e) => println!("Ошибка: {}", e),
            }
        }
        "balance" => {
            if args.len() != 3 {
                eprintln!("Пример: balance John");
                return;
            }
            let name: Name = args[2].clone();
            match storage.get_balance(&name) {
                Some(b) => println!("Баланс {}: {}", name, b),
                None => println!("Пользователь {} не найден", name),
            }
        }
        _ => {
            eprintln!("Неизвестная команда");
        }
    }
}
