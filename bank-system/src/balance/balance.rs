use crate::Balance;
use std::fmt::Display;

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Balance {
    pub fn get(&self) -> i64 {
        self.0
    }
}
