use bank::balance::operations::OperationStatus;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// ## Запись в csv
///
/// ### Поля
/// - TX_ID - ID операции
/// - TX_TYPE - Тип операции
/// - FROM_USER_ID - ID отправителя
/// - TO_USER_ID - ID получателя
/// - AMOUNT - Сумма
/// - TIMESTAMP - Время операции
/// - STATUS - Статус операции
/// - DESCRIPTION - Описание
#[derive(Deserialize, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct CsvRecord {
    pub TX_ID: u64,
    pub TX_TYPE: String,
    pub FROM_USER_ID: u64,
    pub TO_USER_ID: u64,
    pub AMOUNT: u64,
    pub TIMESTAMP: u64,
    pub STATUS: OperationStatus,
    pub DESCRIPTION: String,
}

/// ## Тип файла
///
/// ### Возможные значения
/// - [FileType::CSV] - Csv формат файла
/// - [FileType::TXT] - Txt формат файла
/// - [FileType::BIN] - Bin формат файла
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum FileType {
    /// Csv формат файла
    CSV,
    /// Txt формат файла
    TXT,
    /// Bin формат файла
    BIN,
}

impl TryFrom<String> for FileType {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "csv" => Ok(FileType::CSV),
            "txt" => Ok(FileType::TXT),
            "bin" => Ok(FileType::BIN),
            _ => Err(()),
        }
    }
}
