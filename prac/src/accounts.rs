use std::fmt;
use crate::errors::BankError;

pub trait Account: fmt::Display {
    fn balance(&self) -> f64;
    fn deposit(&mut self, amount: f64) -> Result<(), BankError>;
    fn withdraw(&mut self, amount: f64) -> Result<(), BankError>;
}

#[derive(Debug)]
pub struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    pub fn new(owner: String, initial_balance: f64) -> Self {
        Self {
            owner,
            balance: initial_balance,
        }
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn transfer(&mut self, target: &mut BankAccount, amount: f64) -> Result<(), BankError> {
        self.withdraw(amount)?;
        target.deposit(amount)?;
        Ok(())
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[account] owner: {}, current balance: {:.2}", self.owner, self.balance)
    }
}

impl Account for BankAccount {
    fn balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }
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
    balance: f64,
    credit_limit: f64,
}

impl CreditAccount {
    pub fn new(owner: String, initial_balance: f64, credit_limit: f64) -> Self {
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
            "[credit] owner: {}, balance: {:.2}, credit limit: {:.2}",
            self.owner, self.balance, self.credit_limit
        )
    }
}

impl Account for CreditAccount {
    fn balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }

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
