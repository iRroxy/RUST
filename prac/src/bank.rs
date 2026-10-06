use std::collections::HashMap;
use crate::accounts::{validate_amount, Account, BankAccount, CreditAccount};
use crate::errors::BankError;
use crate::money::Money;

pub struct Bank {
    name: String,
    accounts: HashMap<String, Box<dyn Account>>,
}

impl Bank {
    pub fn new(name: String) -> Self {
        Self {
            name,
            accounts: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn open_account(&mut self, owner: String, initial_balance: Money) {
        let account = BankAccount::new(owner.clone(), initial_balance);
        self.accounts.insert(owner, Box::new(account));
    }

    pub fn open_credit(&mut self, owner: String, initial_balance: Money, credit_limit: Money) {
        let account = CreditAccount::new(owner.clone(), initial_balance, credit_limit);
        self.accounts.insert(owner, Box::new(account));
    }

    pub fn get_balance(&self, owner: &str) -> Result<Money, BankError> {
        match self.accounts.get(owner) {
            Some(account) => Ok(account.balance()),
            None => Err(BankError::AccountNotFound(owner.to_string())),
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: Money) -> Result<(), BankError> {
        // 阶段 1：只检查，不动任何余额
        if from == to {
            return Err(BankError::SameAccount);
        }
        validate_amount(amount)?;
        if !self.accounts.contains_key(from) {
            return Err(BankError::AccountNotFound(from.to_string()));
        }
        if !self.accounts.contains_key(to) {
            return Err(BankError::AccountNotFound(to.to_string()));
        }

        // 阶段 2：取款（失败的话，此时还没有任何改动）
        self.accounts
            .get_mut(from)
            .ok_or_else(|| BankError::AccountNotFound(from.to_string()))?
            .withdraw(amount)?;

        // 取款和存款之间不能写 `?`，否则提前返回会丢钱
        let deposit_result = match self.accounts.get_mut(to) {
            Some(dst) => dst.deposit(amount),
            None => Err(BankError::AccountNotFound(to.to_string())),
        };

        // 阶段 3：存款失败就回滚，把钱退给转出账户
        if let Err(e) = deposit_result {
            self.accounts
                .get_mut(from)
                .expect("source account was withdrawn from a moment ago")
                .deposit(amount)
                .expect("refunding a validated amount cannot fail");
            return Err(e);
        }

        Ok(())
    }

    pub fn total_assets(&self) -> Money {
        self.accounts.values().map(|acc| acc.balance()).sum()
    }

    pub fn get_vip_accounts(&self, min_balance: Money) -> Vec<&dyn Account> {
        self.accounts
            .values()
            .map(|acc| acc.as_ref())
            .filter(|acc| acc.balance() >= min_balance)
            .collect()
    }
}

#[cfg(test)]
mod tests {
use std::fmt;
use super::*;

    #[test]
    fn test_open_account_and_balance() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());

        let balance = bank.get_balance("Alice").unwrap();
        assert_eq!(balance, Money::from_yuan(100).unwrap());
    }

    #[test]
    fn test_successful_transfer() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());
        bank.open_account("Bob".to_string(), Money::from_yuan(50).unwrap());

        let result = bank.transfer("Alice", "Bob", Money::from_yuan(40).unwrap());
        assert!(result.is_ok());

        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(60).unwrap());
        assert_eq!(bank.get_balance("Bob").unwrap(), Money::from_yuan(90).unwrap());
    }

    #[test]
    fn test_insufficient_funds_transfer() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());
        bank.open_account("Bob".to_string(), Money::from_yuan(50).unwrap());

        let result = bank.transfer("Alice", "Bob", Money::from_yuan(200).unwrap());
        assert!(result.is_err());

        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(100).unwrap());
        assert_eq!(bank.get_balance("Bob").unwrap(), Money::from_yuan(50).unwrap());
    }

    #[test]
    fn test_credit_account_overdraft() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_credit("Bob".to_string(), Money::ZERO, Money::from_yuan(500).unwrap());
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());

        // 1. Bob 透支 300 成功
        assert!(bank.transfer("Bob", "Alice", Money::from_yuan(300).unwrap()).is_ok());
        assert_eq!(bank.get_balance("Bob").unwrap(), Money::from_yuan(-300).unwrap());
        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(400).unwrap());

        // 2. Bob 再次透支 300 失败（总计 600 > 500 额度）
        assert!(bank.transfer("Bob", "Alice", Money::from_yuan(300).unwrap()).is_err());
        assert_eq!(bank.get_balance("Bob").unwrap(), Money::from_yuan(-300).unwrap());
        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(400).unwrap());
    }

    #[test]
    fn test_transfer_to_self() {
        let mut bank = Bank::new(String::from("Iron Bank"));

        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());
        let result = bank.transfer("Alice", "Alice", Money::from_yuan(50).unwrap());
        assert_eq!(result, Err(BankError::SameAccount));
        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(100).unwrap());
    }

    #[test]
    fn test_transfer_to_nonexistent_account() {
        let mut bank = Bank::new(String::from("Eris Bank"));
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());
        let result = bank.transfer("Alice", "Charlie", Money::from_yuan(100).unwrap());
        assert_eq!(result, Err(BankError::AccountNotFound("Charlie".to_string())));
        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(100).unwrap());
    }
    #[test]
    fn test_vip_and_total_assets() {
        let mut bank = Bank::new(String::from("Eris Bank"));
        bank.open_account("Alice".to_string(), Money::from_yuan(1000).unwrap());
        bank.open_account("Bob".to_string(), Money::from_yuan(500).unwrap());
        bank.open_account("Charlie".to_string(), Money::from_yuan(200).unwrap());

        assert_eq!(bank.total_assets(), Money::from_yuan(1700).unwrap());

        let vips = bank.get_vip_accounts(Money::from_yuan(500).unwrap());
        assert_eq!(vips.len(), 2);
        assert!(vips.iter().all(|acc| acc.balance() >= Money::from_yuan(500).unwrap()));
    }

    #[test]
    fn test_transfer_from_nonexistent_account() {
        let mut bank = Bank::new(String::from("Eris Bank"));
        bank.open_account("Bob".to_string(), Money::from_yuan(100).unwrap());
        let result = bank.transfer("Charlie", "Bob", Money::from_yuan(50).unwrap());
        assert_eq!(result, Err(BankError::AccountNotFound("Charlie".to_string())));
        assert_eq!(bank.get_balance("Bob").unwrap(), Money::from_yuan(100).unwrap());
    }

    #[test]
    fn test_transfer_invalid_amount() {
        let mut bank = Bank::new(String::from("Iron Bank"));
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());
        bank.open_account("Bob".to_string(), Money::from_yuan(50).unwrap());

        assert_eq!(bank.transfer("Alice", "Bob", Money::from_yuan(-20).unwrap()), Err(BankError::InvalidAmount(Money::from_yuan(-20).unwrap())));
        assert_eq!(bank.transfer("Alice", "Bob", Money::ZERO), Err(BankError::InvalidAmount(Money::ZERO)));
        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(100).unwrap());
        assert_eq!(bank.get_balance("Bob").unwrap(), Money::from_yuan(50).unwrap());
    }

    struct BrokenAccount;

    impl fmt::Display for BrokenAccount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[broken account]")
        }
    }

    impl Account for BrokenAccount {
        fn balance(&self) -> Money {
            Money::ZERO
        }

        fn deposit(&mut self, amount: Money) -> Result<(), BankError> {
            Err(BankError::InvalidAmount(amount))
        }

        fn withdraw(&mut self, _amount: Money) -> Result<(), BankError> {
            Ok(())
        }
    }

    #[test]
    fn test_transfer_rollback_on_deposit_failure() {
        let mut bank = Bank::new(String::from("Iron Bank"));
        bank.open_account("Alice".to_string(), Money::from_yuan(100).unwrap());
        bank.accounts.insert("Broken".to_string(), Box::new(BrokenAccount));

        let result = bank.transfer("Alice", "Broken", Money::from_yuan(30).unwrap());
        assert!(result.is_err());
        assert_eq!(bank.get_balance("Alice").unwrap(), Money::from_yuan(100).unwrap());
    }
}

