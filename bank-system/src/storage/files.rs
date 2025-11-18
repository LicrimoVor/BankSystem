use super::Storage;
use crate::balance::{Balance, BalanceOpError};
use std::{
    fs::{self, File},
    io::{self, BufRead, BufWriter},
    path::Path,
};

impl Storage {
    fn set_balance(&mut self, name: &str, balance: Balance) {
        self.accounts
            .entry(name.to_string())
            .and_modify(|b| *b = balance.clone())
            .or_insert(balance);
    }
    pub fn load_data(file: &str) -> Result<Storage, std::io::Error> {
        let mut storage = Storage::new();
        if !Path::new(file).exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Файл не найден",
            ));
        }

        let file = File::open(file)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            // Каждая строка — это Result<String>, поэтому делаем if let Ok
            if let Ok(line) = line {
                // Разделяем строку по запятой: "Name,Balance"
                let parts: Vec<&str> = line.trim().split(';').collect();

                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    // Пробуем преобразовать баланс из строки в число
                    let balance = Balance::try_from(parts[1].to_string()).map_err(|e| {
                        let message = if let BalanceOpError::ParseError(e) = e {
                            format!("Неверный формат баланса: {}", e)
                        } else {
                            "Неверный формат баланса".to_string()
                        };
                        std::io::Error::new(std::io::ErrorKind::InvalidData, message)
                    })?;

                    // Добавляем пользователя и выставляем баланс
                    storage.add_user(name.clone());
                    storage.set_balance(&name, balance);
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Неверный формат строки",
                    ));
                }
            }
        }

        Ok(storage)
    }

    pub fn save(&self, file: &str) {
        //// первый способ сохранения
        let mut data = String::new();

        for (name, balance) in self.get_all() {
            data.push_str(&format!("{};{}\n", name, balance));
        }
        fs::write(file, data).expect("Не удалось записать файл");

        //// второй способ сохранения
        // let f = File::create("data.csv").unwrap();
        // let mut writer = BufWriter::new(f);

        // for (name, balance) in self.get_all() {
        //     writeln!(writer, "{};{}", name, balance);
        // }
        // writer.flush().unwrap(); // всё записано в файл одной операцией
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::balance::BalanceManager;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_storage_is_empty() {
        let bank = Storage::new();
        assert_eq!(bank.accounts.len(), 0);
    }

    #[test]
    fn test_load_data_existing_file() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "John;100,[D100]\nAlice;200,[D300,W100]\n").unwrap();
        let path = file.path().to_str().unwrap();
        let storage = Storage::load_data(path);
        assert!(storage.is_ok());

        let storage = storage.unwrap();
        let mut j_balance = Balance::new(0);
        j_balance += 100;
        let mut a_balance = Balance::new(0);
        a_balance += 300;
        a_balance -= 100;

        assert_eq!(storage.get_balance(&"John".to_string()), Some(&j_balance));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(&a_balance));
    }

    #[test]
    fn test_load_data_not_existing_file() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "John;100,[D100, 100]\nAlice;200,[D300,W100]\n").unwrap();
        let path = file.path().to_str().unwrap();

        let storage = Storage::load_data(path);
        assert!(storage.is_err());
    }

    #[test]
    fn test_save_creates_file_with_correct_data() {
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();

        storage.deposit(&"Alice".to_string(), 300).unwrap();
        storage.withdraw(&"Alice".to_string(), 100).unwrap();
        storage.save("test.csv");

        let data = fs::read_to_string("test.csv").unwrap();
        let mut lines: Vec<&str> = data.lines().collect();
        lines.sort();
        assert_eq!(lines, vec!["Alice;200,[D300,W100]", "John;150,[D150]"]);

        fs::remove_file("test.csv").unwrap();
    }
}
