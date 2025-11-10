use crate::{Balance, Command, Name};

impl Command {
    pub fn add_user(name: Name) -> Self {
        Command::AddUser(name)
    }

    pub fn remove_user(name: Name) -> Self {
        Command::RemoveUser(name)
    }

    pub fn deposit(name: Name, amount: Balance) -> Self {
        Command::Deposit(name, amount)
    }

    pub fn withdraw(name: Name, amount: Balance) -> Self {
        Command::Withdraw(name, amount)
    }

    pub fn balance(name: Name) -> Self {
        Command::Balance(name)
    }

    pub fn transfer(from: Name, to: Name, amount: Balance) -> Self {
        Command::Transfer(from, to, amount)
    }
}
