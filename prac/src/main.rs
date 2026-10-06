#![allow(dead_code)]

use prac::bank::Bank;
use prac::money::Money;

fn main() {
    println!("\n==== 工程化多模块银行系统测试 ====");
    let mut bank = Bank::new(String::from("Iron Bank"));

    bank.open_account(String::from("Alice"), Money::from_yuan(100).unwrap());
    bank.open_credit(String::from("Bob"), Money::ZERO, Money::from_yuan(500).unwrap());

    println!("Alice 初始余额: {}", bank.get_balance("Alice").unwrap());
    println!("Bob 初始余额: {}", bank.get_balance("Bob").unwrap());

    println!("\n-- 测试：Bob 尝试透支转账 300 元给 Alice --");
    match bank.transfer("Bob", "Alice", Money::from_yuan(300).unwrap()) {
        Ok(()) => println!("转账成功！"),
        Err(e) => println!("转账失败: {e}"),
    }

    println!("Alice 当前余额: {}", bank.get_balance("Alice").unwrap());
    println!("Bob 当前余额: {}", bank.get_balance("Bob").unwrap());

    println!("\n-- 测试：Bob 再次尝试透支 300 元（额度超标拦截）--");
    match bank.transfer("Bob", "Alice", Money::from_yuan(300).unwrap()) {
        Ok(()) => println!("转账成功"),
        Err(e) => println!("捕获到预期错误: {e}"),
    }

    println!("\n全行总资产: {}", bank.total_assets());
}
