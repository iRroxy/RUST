use std::fmt;
use crate::money::Money;

#[derive(Debug, PartialEq)]
pub enum BankError {
    InsufficientFunds { balance: Money, required: Money },
    InvalidAmount(Money),
    AccountNotFound(String),
    SameAccount,
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BankError::InsufficientFunds { balance, required } => {
                write!(f, "not enough!: {balance}, try to withdraw {required}")
            }
            BankError::InvalidAmount(amount) => {
                write!(f, "error! {amount}")
            }
            BankError::AccountNotFound(name) => {
                write!(f, "account not found! {name}")
            }
            BankError::SameAccount => {
                write!(f, "cannot transfer to the same account")
            }
        }
    }
}
