/// Ошибки (все возможные ошибки в проекте)
#[derive(Debug, Clone, PartialEq)]
pub enum QuoteError {
    NotFound,
    BadRequest(String),
    InternalError,
    NotConnection,
    AlreadyExists,
    KeyNotEqual,
    Other(String),
}

impl QuoteError {
    pub fn to_string(self) -> String {
        match self {
            QuoteError::NotFound => "NotFound".to_string(),
            QuoteError::BadRequest(e) => format!("BadRequest {}", e),
            QuoteError::InternalError => "InternalError".to_string(),
            QuoteError::NotConnection => "NotConnection".to_string(),
            QuoteError::AlreadyExists => "AlreadyExists".to_string(),
            QuoteError::KeyNotEqual => "KeyNotEqual".to_string(),
            QuoteError::Other(e) => format!("Other {}", e),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, ()> {
        if s.len() < 5 {
            return Err(());
        }

        println!("{}", s);
        match &s[..5] {
            "NotFo" => Ok(QuoteError::NotFound),
            "BadRe" => Ok(QuoteError::BadRequest(s[11..].to_string())),
            "Inter" => Ok(QuoteError::InternalError),
            "NotCo" => Ok(QuoteError::NotConnection),
            "Alrea" => Ok(QuoteError::AlreadyExists),
            "KeyNo" => Ok(QuoteError::KeyNotEqual),
            "Other" => Ok(QuoteError::Other(s[6..].to_string())),
            _ => Err(()),
        }
    }
}
