use super::OperationError;

/// Статус операции
#[derive(Debug, Clone)]
pub enum Status {
    FAILURE,
    PENDING,
    SUCCESS,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Status::FAILURE => "failure",
            Status::PENDING => "pending",
            Status::SUCCESS => "success",
        };
        write!(f, "{}", name)
    }
}

impl TryFromom<String> for Status {
    type Error = OperationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "failure" => Ok(Status::FAILURE),
            "pending" => Ok(Status::PENDING),
            "success" => Ok(Status::SUCCESS),
            _ => Err(OperationError::InvalidStatus),
        }
    }
}
