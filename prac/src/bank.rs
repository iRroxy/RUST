use std::collections::HashMap;
use crate::accounts::{Account, BankAccount, CreditAccount};
use crate::errors::BankError;

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

    pub fn open_account(&mut self, owner: String, initial_balance: f64) {
        let account = BankAccount::new(owner.clone(), initial_balance);
        self.accounts.insert(owner, Box::new(account));
    }

    pub fn open_credit(&mut self, owner: String, initial_balance: f64, credit_limit: f64) {
        let account = CreditAccount::new(owner.clone(), initial_balance, credit_limit);
        self.accounts.insert(owner, Box::new(account));
    }

    pub fn get_balance(&self, owner: &str) -> Result<f64, BankError> {
        match self.accounts.get(owner) {
            Some(account) => Ok(account.balance()),
            None => Err(BankError::AccountNotFound(owner.to_string())),
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: f64) -> Result<(), BankError> {
        if from == to {
            return Err(BankError::InvalidAmount(amount));
        }
        if !self.accounts.contains_key(to) {
            return Err(BankError::AccountNotFound(to.to_string()));
        }

        let from_account = self
            .accounts
            .get_mut(from)
            .ok_or_else(|| BankError::AccountNotFound(from.to_string()))?;
        from_account.withdraw(amount)?;

        let to_account = self
            .accounts
            .get_mut(to)
            .ok_or_else(|| BankError::AccountNotFound(to.to_string()))?;
        to_account.deposit(amount)?;

        Ok(())
    }

    pub fn total_assets(&self) -> f64 {
        self.accounts.values().map(|acc| acc.balance()).sum()
    }

    pub fn get_vip_accounts(&self, min_balance: f64) -> Vec<&Box<dyn Account>> {
        self.accounts
            .values()
            .filter(|acc| acc.balance() >= min_balance)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::bank;

use super::*;

    #[test]
    fn test_open_account_and_balance() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_account("Alice".to_string(), 100.0);

        let balance = bank.get_balance("Alice").unwrap();
        assert_eq!(balance, 100.0);
    }

    #[test]
    fn test_successful_transfer() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_account("Alice".to_string(), 100.0);
        bank.open_account("Bob".to_string(), 50.0);

        let result = bank.transfer("Alice", "Bob", 40.0);
        assert!(result.is_ok());

        assert_eq!(bank.get_balance("Alice").unwrap(), 60.0);
        assert_eq!(bank.get_balance("Bob").unwrap(), 90.0);
    }

    #[test]
    fn test_insufficient_funds_transfer() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_account("Alice".to_string(), 100.0);
        bank.open_account("Bob".to_string(), 50.0);

        let result = bank.transfer("Alice", "Bob", 200.0);
        // 转账必须失败
        assert!(result.is_err());

        // 验证资金没有被扣除（原子性，不会出现 Alice 扣钱了 Bob 没收到的情况）
        assert_eq!(bank.get_balance("Alice").unwrap(), 100.0);
        assert_eq!(bank.get_balance("Bob").unwrap(), 50.0);
    }

    #[test]
    fn test_credit_account_overdraft() {
        let mut bank = Bank::new("Iron Bank".to_string());
        bank.open_credit("Bob".to_string(), 0.0, 500.0);
        bank.open_account("Alice".to_string(), 100.0);

        // 1. Bob 透支 300 成功
        assert!(bank.transfer("Bob", "Alice", 300.0).is_ok());
        assert_eq!(bank.get_balance("Bob").unwrap(), -300.0);
        assert_eq!(bank.get_balance("Alice").unwrap(), 400.0);

        // 2. Bob 再次透支 300 失败（总计 600 > 500 额度）
        assert!(bank.transfer("Bob", "Alice", 300.0).is_err());
        assert_eq!(bank.get_balance("Bob").unwrap(), -300.0);
        assert_eq!(bank.get_balance("Alice").unwrap(), 400.0);
    }

    #[test]
    fn test_transfer_to_self() {
        let mut bank = Bank::new(String::from("Iron Bank"));

        bank.open_account("Alice".to_string(), 100.0);
        let result = bank.transfer("Alice", "Alice", 50.0);
        assert!(result.is_err());

        assert_eq!(bank.get_balance("Alice").unwrap(), 100.0);

    }

    #[test]
    fn test_transfer_to_nonexistent_account() {
        let mut bank = Bank::new(String::from("Eris Bank"));
        bank.open_account("Alice".to_string(), 100.0);
        let result = bank.transfer("Alice", "Charlie", 100.0);
        assert!(result.is_err());
        assert_eq!(bank.get_balance("Alice").unwrap(), 100.0);
        
    }
    #[test]
    fn test_vip_and_total_assets() {
        let mut bank = Bank::new(String::from("Eris Bank"));
        bank.open_account("Alice".to_string(), 1000.0);
        bank.open_account("Bob".to_string(), 500.0);
        bank.open_account("Charlie".to_string(), 200.0);

        assert_eq!(bank.total_assets(), 1700.0);

        let vips = bank.get_vip_accounts(500.0);
        assert_eq!(vips.len(), 2);
    }
}

