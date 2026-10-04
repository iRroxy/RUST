use prac::bank::Bank;

#[test]
fn test_bank_end_to_end_flow() {
    let mut bank = Bank::new(String::from("Federal Bank"));
    bank.open_account(String::from("Alice"), 500.0);
    bank.open_credit(String::from("Bob"), 100.0, 1000.0);

    let result = bank.transfer("Bob", "Alice", 300.0);
    assert!(result.is_ok());

    assert_eq!(bank.get_balance("Alice").unwrap(), 800.0);
    assert_eq!(bank.get_balance("Bob").unwrap(), -200.0);
    assert_eq!(bank.total_assets(), 600.0)
}