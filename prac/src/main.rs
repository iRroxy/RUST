#![warn(dead_code)]
use std::fmt;

#[derive(Debug)]
pub enum BankError {
    InsufficientFunds { balance: f64 , required: f64 },
    InvalidAmount(f64),
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
        }
    }
}

#[derive(Debug)]
pub struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    pub fn new(owner: String, initial_balance: f64) -> Self {
        Self { owner, balance: initial_balance, }
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return  Err(BankError::InvalidAmount(amount));
        }
        self.balance += amount;
        Ok(())
    }
    pub fn withdraw(&mut self, amount: f64) -> Result<(), BankError> {
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount(amount));
        }
        if self.balance < amount {
            return Err(BankError::InsufficientFunds { balance: self.balance, required: amount });
        }
        self.balance -= amount;
        Ok(())
    }


    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[account]owner: {},current balance: {:.2}", self.owner, self.balance )
    }
}

impl BankAccount {
    pub fn transfer(&mut self, target: &mut BankAccount, amount: f64) -> Result<(), BankError> {
        self.withdraw(amount)?;

        target.deposit(amount)?;

        Ok(())
    }
}

fn main() {
    let mut my_account = BankAccount::new(String::from("Eris"), 100.0);
    println!("initial state -> {my_account}");

    if let Err(e) = my_account.deposit(50.0) {
        println!("save money refuse: {e}");
    }

    match my_account.withdraw(80.0) {
        Ok(()) => println!("successfully withdraw"),
        Err(e) => println!("unsuccessfully withdraw"),
    }
    println!("after -> {my_account}");

    println!("\n---- test");
    match  my_account.withdraw(200.0) {
        Ok(()) => println!("success"),
        Err(e) => println!("catch the error: {e}"),
    }

    println!("\n---- test");
    if let Err(e) =  my_account.deposit(20.0) {
        println!("catch! {e}");
    }

    println!("final account state -> {my_account} ");
}