use crate::Storage;
use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

impl Storage {
    /// Загружает данные из CSV-файла или создаёт хранилище с дефолтными пользователями
    pub fn load_data(file: &str) -> Storage {
        let mut storage = Storage::new();

        // Проверяем, существует ли файл
        if Path::new(file).exists() {
            // Открываем файл
            let file = File::open(file).unwrap();

            // Оборачиваем файл в BufReader
            // BufReader читает данные блоками и хранит их в буфере,
            // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту
            let reader = io::BufReader::new(file);

            // Читаем файл построчно
            for line in reader.lines() {
                // Каждая строка — это Result<String>, поэтому делаем if let Ok
                if let Ok(line) = line {
                    // Разделяем строку по запятой: "Name,Balance"
                    let parts: Vec<&str> = line.trim().split(',').collect();

                    if parts.len() == 2 {
                        let name = parts[0].to_string();
                        // Пробуем преобразовать баланс из строки в число
                        let balance: i64 = parts[1].parse().unwrap_or(0);

                        // Добавляем пользователя и выставляем баланс
                        storage.add_user(name.clone());
                        let _ = storage.deposit(&name, balance.into());
                    }
                }
            }
        } else {
            // если файла нет, создаём пользователей с нуля
            for u in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(u.to_string());
            }
        }

        storage
    }

    /// Сохраняет текущее состояние Storage в CSV-файл
    pub fn save(&self, file: &str) {
        let mut data = String::new();

        // Собираем все данные в одну строку формата "Name,Balance"
        for (name, balance) in self.get_all() {
            data.push_str(&format!("{},{}\n", name, balance.get()));
        }

        // Записываем в файл
        // Здесь мы не используем BufWriter, потому что сразу пишем всю строку целиком.
        fs::write(file, data).expect("Не удалось записать файл");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{BufReader, BufWriter, Cursor, Write};

    #[test]
    fn test_new_storage_is_empty() {
        let bank = Storage::new();
        assert_eq!(bank.accounts.len(), 0);
    }

    #[test]
    fn test_load_data() {
        let f = File::create("data.csv").unwrap();
        let mut writer = BufWriter::new(f);

        writeln!(writer, "John,100").unwrap(); // пока в буфере
        writeln!(writer, "Alice,200").unwrap(); // пока в буфере
        writer.flush().unwrap(); // всё записано в файл одной операцией 
    }

    #[test]
    fn test_load_data_existing_file() {
        let mut file = File::create("test.csv").unwrap();
        // file.write_all(b"John,100\nAlice,200\nBob,50\n").unwrap();
        writeln!(file, "John,100").unwrap();
        writeln!(file, "Alice,200").unwrap();
        writeln!(file, "Bob,50").unwrap();

        let storage = Storage::load_data("test.csv");

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100.into()));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200.into()));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50.into()));
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);

        fs::remove_file("test.csv").unwrap();
    }

    #[test]
    fn test_load_data_existing_cursor() {
        // Создаём данные в памяти, как будто это CSV-файл
        let data = b"John,100\nAlice,200\nBob,50\n";
        let mut cursor = Cursor::new(&data[..]);

        // Читаем данные из Cursor
        let mut storage = Storage::new();
        let reader = BufReader::new(&mut cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let balance: i64 = parts[1].parse().unwrap_or(0);
                storage.add_user(name.clone());
                storage.deposit(&name, balance.into()).unwrap();
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100.into()));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200.into()));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50.into()));
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None); // нет в данных
    }

    #[test]
    fn test_save_creates_file_with_correct_data() {
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150.into()).unwrap();
        storage.deposit(&"Alice".to_string(), 300.into()).unwrap();

        storage.save("test.csv");

        let data = fs::read_to_string("test.csv").unwrap();
        let mut lines: Vec<&str> = data.lines().collect();
        lines.sort();

        assert_eq!(lines, vec!["Alice,300", "John,150"]);

        fs::remove_file("test.csv").unwrap();
    }
}
