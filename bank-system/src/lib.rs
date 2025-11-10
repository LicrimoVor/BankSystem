use derive_more::{Add, AddAssign, From, Into, Mul, MulAssign, Sub, SubAssign};
use std::collections::HashMap;
pub mod balance;
pub mod command;
pub mod storage;

pub type Name = String;

#[derive(
    Debug,
    Clone,
    Copy,
    Add,
    AddAssign,
    Mul,
    MulAssign,
    From,
    Into,
    PartialEq,
    Eq,
    Sub,
    SubAssign,
    PartialOrd,
    Ord,
)]
pub struct Balance(i64);

pub struct Storage {
    accounts: HashMap<Name, Balance>,
}

pub enum Command {
    AddUser(Name),
    RemoveUser(Name),
    Balance(Name),
    Deposit(Name, Balance),
    Withdraw(Name, Balance),
    Transfer(Name, Name, Balance),
}
