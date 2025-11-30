use super::errors::ParseFromFileError;
use crate::OperationName;
use bank::balance::operations::{Operation, OperationStatus, OperationType};

pub fn parse_from_bin<R: std::io::Read>(
    r: &mut R,
) -> Result<Vec<OperationName>, ParseFromFileError> {
    let mut bytes = Vec::new();
    r.read_to_end(&mut bytes)
        .or_else(|e| Err(ParseFromFileError::IoError(e)))?;

    let data: Vec<&[u8]> = {
        let mut i = 0;
        let bytes_len = bytes.len();
        bytes
            .split(|_| {
                i += 1;
                if bytes_len < i + 4 {
                    false
                } else {
                    bytes[i..i + 4] == *"YPBN".as_bytes()
                }
            })
            .collect()
    };

    let operations: Vec<Result<OperationName, ParseFromFileError>> = data
        .iter()
        .map(|balance| {
            let mut i = 0;
            let _magic = {
                let end = i + 4;
                let arr: [u8; 4] = balance[i..end].try_into().expect("REASON");
                i = end;
                str::from_utf8(&arr).expect("REASON").to_string()
            };
            let _size = {
                let end = i + 4;
                let arr: [u8; 4] = balance[i..end].try_into().expect("REASON");
                i = end;
                u32::from_be_bytes(arr)
            };
            let id = {
                let end = i + 8;
                let arr: [u8; 8] = balance[i..end].try_into().expect("REASON");
                i = end;
                u64::from_be_bytes(arr)
            };
            let tx_type = {
                let res = match balance[i] {
                    0 => Ok(OperationType::Deposit(0)),
                    1 => Ok(OperationType::Transfer("".to_string(), 0, false)),
                    2 => Ok(OperationType::Withdraw(0)),
                    _ => Err(()),
                };
                i += 1;

                res
            }
            .or(Err(ParseFromFileError::ParseError(
                "Неверный тип операции".to_string(),
            )))?;
            let from_user = {
                let end = i + 8;
                let arr: [u8; 8] = balance[i..end].try_into().expect("REASON");
                i = end;
                u64::from_be_bytes(arr)
            };
            let to_user = {
                let end = i + 8;
                let arr: [u8; 8] = balance[i..end].try_into().expect("REASON");
                i = end;
                u64::from_be_bytes(arr)
            };
            let amount = {
                let end = i + 8;
                let arr: [u8; 8] = balance[i..end].try_into().expect("REASON");
                i = end;
                i64::from_be_bytes(arr)
            };
            let timestamp = {
                let end = i + 8;
                let arr: [u8; 8] = balance[i..end].try_into().expect("REASON");
                i = end;
                u64::from_be_bytes(arr)
            };
            let status = {
                i += 1;
                match balance[i] {
                    0 => Ok(OperationStatus::SUCCESS),
                    1 => Ok(OperationStatus::FAILURE),
                    2 => Ok(OperationStatus::PENDING),
                    _ => Err(()),
                }
            }
            .or(Err(ParseFromFileError::ParseError(
                "Неверный статус операции".to_string(),
            )))?;
            let desc_len = {
                let end = i + 4;
                let arr: [u8; 4] = balance[i..end].try_into().expect("REASON");
                i = end;
                u32::from_be_bytes(arr)
            };
            let desc = {
                let end = i + desc_len as usize - 1;
                let arr: Vec<u8> = balance[i..end].to_vec();
                String::from_utf8(arr).expect("REASON")
            };

            let tx_type = match tx_type {
                OperationType::Deposit(_) => OperationType::Deposit(amount.abs() as u64),
                OperationType::Withdraw(_) => OperationType::Withdraw(amount.abs() as u64),
                OperationType::Transfer(_, _, _) => {
                    OperationType::Transfer(from_user.to_string(), amount.abs() as u64, true)
                }
                OperationType::Close => OperationType::Close,
            };
            let operation = Operation::load(id, timestamp, tx_type, status, Some(desc));
            let name = if from_user == 0 { to_user } else { from_user };

            Ok(OperationName(operation, name.to_string()))
        })
        .collect();

    operations.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::BufReader};
    const PATH_TEST: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/test.bin");

    #[test]
    fn test_parse_from_bin_success() {
        let file = File::open(PATH_TEST).unwrap();
        let mut buf = BufReader::new(file);
        let res = parse_from_bin(&mut buf);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 1000);
    }
}
