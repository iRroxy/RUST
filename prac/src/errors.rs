use crate::money::Money;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum BankError {
    InsufficientFunds { balance: Money, required: Money },
    InvalidAmount(Money),
    AccountNotFound(String),
    SameAccount,
    Overflow,
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
            BankError::Overflow => {
                write!(f, "arithmetic overflow occurred")
            }
        }
    }
}
