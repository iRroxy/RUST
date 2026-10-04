use std::fmt;

#[derive(Debug)]
pub enum BankError {
    InsufficientFunds { balance: f64, required: f64 },
    InvalidAmount(f64),
    AccountNotFound(String),
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { balance, required } => {
                write!(f, "not enough!: {:.2}, try to withdraw {:.2}", balance, required)
            }
            BankError::InvalidAmount(amount) => {
                write!(f, "error! {:.2}", amount)
            }
            BankError::AccountNotFound(name) => {
                write!(f, "account not found! {name}")
            }
        }
    }
}
