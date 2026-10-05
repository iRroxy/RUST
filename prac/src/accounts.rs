use std::fmt;
use crate::errors::BankError;
use crate::money::Money;

pub trait Account: fmt::Display {
    fn balance(&self) -> Money;
    fn deposit(&mut self, amount: Money) -> Result<(), BankError>;
    fn withdraw(&mut self, amount: Money) -> Result<(), BankError>;
}

pub(crate) fn validate_amount(amount: Money) -> Result<(), BankError> {
    if amount <= Money::ZERO {
        Err(BankError::InvalidAmount(amount))
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct BankAccount {
    owner: String,
    balance: Money,
}

impl BankAccount {
    pub fn new(owner: String, initial_balance: Money) -> Self {
        Self {
            owner,
            balance: initial_balance,
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn transfer(&mut self, target: &mut BankAccount, amount: Money) -> Result<(), BankError> {
        self.withdraw(amount)?;
        target.deposit(amount)?;
        Ok(())
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[account] owner: {}, current balance: {}", self.owner, self.balance)
    }
}

impl Account for BankAccount {
    fn balance(&self) -> Money {
        self.balance
    }

    fn deposit(&mut self, amount: Money) -> Result<(), BankError> {
        validate_amount(amount)?;
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: Money) -> Result<(), BankError> {
        validate_amount(amount)?;
        if self.balance < amount {
            return Err(BankError::InsufficientFunds {
                balance: self.balance,
                required: amount,
            });
        }
        self.balance -= amount;
        Ok(())
    }
}

pub struct CreditAccount {
    owner: String,
    balance: Money,
    credit_limit: Money,
}

impl CreditAccount {
    pub fn new(owner: String, initial_balance: Money, credit_limit: Money) -> Self {
        Self {
            owner,
            balance: initial_balance,
            credit_limit,
        }
    }
}

impl fmt::Display for CreditAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[credit] owner: {}, balance: {}, credit limit: {}",
            self.owner, self.balance, self.credit_limit
        )
    }
}

impl Account for CreditAccount {
    fn balance(&self) -> Money {
        self.balance
    }

    fn deposit(&mut self, amount: Money) -> Result<(), BankError> {
        validate_amount(amount)?;
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: Money) -> Result<(), BankError> {
        validate_amount(amount)?;

        let available = self.balance + self.credit_limit;
        if available < amount {
            return Err(BankError::InsufficientFunds {
                balance: available,
                required: amount,
            });
        }
        self.balance -= amount;
        Ok(())
    }
}
