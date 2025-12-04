use bank::balance::operations::OperationStatus;
use serde::{Deserialize, Serialize};

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
