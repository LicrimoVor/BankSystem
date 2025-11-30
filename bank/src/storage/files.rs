use super::Storage;
use crate::balance::{Balance, errors::BalanceError};
use std::{
    fs::{self, File},
    io::{self, BufRead},
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
                        let message = if let BalanceError::InvalidParseBalance(e) = e {
                            format!("Неверный формат баланса: {}", e)
                        } else {
                            "Неверный формат операций".to_string()
                        };
                        std::io::Error::new(std::io::ErrorKind::InvalidData, message)
                    })?;

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
        let mut data = String::new();
        for (name, balance) in self.get_all() {
            data.push_str(&format!("{};{}\n", name, balance.save()));
        }
        fs::write(file, data).expect("Не удалось записать файл");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        write!(file,
"Ivan;300,[1,1764444526,D100,success,Record number #1|3,1764444535,T(Julia:200:true),success,Record number #3]
Julia;400,[2,1764444530,D600,success,Record number #2|3,1764444535,T(Ivan:200:false),success,Record number #3]").unwrap();
        let path = file.path().to_str().unwrap();
        let storage = Storage::load_data(path);
        println!("{:?}", storage);
        assert!(storage.is_ok());

        let storage = storage.unwrap();

        let j_balance = storage.get_balance(&"Ivan".to_string());
        let a_balance = storage.get_balance(&"Julia".to_string());

        assert!(j_balance.is_some());
        assert!(a_balance.is_some());

        let j_balance = j_balance.unwrap();
        let a_balance = a_balance.unwrap();

        assert_eq!(j_balance.get_value(), 300);
        assert_eq!(a_balance.get_value(), 400);
    }

    #[test]
    fn test_load_data_not_existing_file() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            "Ivan;300,[1,1764444526,O100,success,Record number #1]\n"
        )
        .unwrap();
        let path = file.path().to_str().unwrap();

        let storage = Storage::load_data(path);
        assert!(storage.is_err());
    }
}
