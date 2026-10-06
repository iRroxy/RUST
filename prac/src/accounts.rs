use crate::errors::BankError;
use crate::money::Money;
use std::fmt;

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
        write!(
            f,
            "[account] owner: {}, current balance: {}",
            self.owner, self.balance
        )
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
        let new_balance = self
            .balance
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        self.balance = new_balance;
        Ok(())
    }

    fn withdraw(&mut self, amount: Money) -> Result<(), BankError> {
        validate_amount(amount)?;

        let available = self
            .balance
            .checked_add(self.credit_limit)
            .ok_or(BankError::Overflow)?;
        if available < amount {
            return Err(BankError::InsufficientFunds {
                balance: available,
                required: amount,
            });
        }
        let new_balance = self
            .balance
            .checked_sub(amount)
            .ok_or(BankError::Overflow)?;
        self.balance = new_balance;
        Ok(())
    }
}

impl Account for BankAccount {
    fn balance(&self) -> Money {
        self.balance
    }
    fn deposit(&mut self, amount: Money) -> Result<(), BankError> {
        validate_amount(amount)?;
        let new_balance = self
            .balance
            .checked_add(amount)
            .ok_or(BankError::Overflow)?;
        self.balance = new_balance;
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
        let new_balance = self
            .balance
            .checked_sub(amount)
            .ok_or(BankError::Overflow)?;
        self.balance = new_balance;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_account_deposit_overflow() {
        let mut account =
            CreditAccount::new("Bob".to_string(), Money::from_cents(i64::MAX), Money::ZERO);
        let result = account.deposit(Money::from_cents(1));
        assert_eq!(result, Err(BankError::Overflow));
        assert_eq!(account.balance(), Money::from_cents(i64::MAX));
    }

    #[test]
    fn test_credit_account_withdraw_available_overflow() {
        let mut account = CreditAccount::new(
            "Bob".to_string(),
            Money::from_cents(1),
            Money::from_cents(i64::MAX),
        );
        let result = account.withdraw(Money::from_cents(100));
        assert_eq!(result, Err(BankError::Overflow));
        assert_eq!(account.balance(), Money::from_cents(1));
    }
}
