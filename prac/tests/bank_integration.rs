use prac::bank::Bank;
use prac::money::Money;

#[test]
fn test_bank_end_to_end_flow() {
    let mut bank = Bank::new(String::from("Federal Bank"));
    bank.open_account(String::from("Alice"), Money::from_yuan(500).unwrap())
        .unwrap();
    bank.open_credit(
        String::from("Bob"),
        Money::from_yuan(100).unwrap(),
        Money::from_yuan(1000).unwrap(),
    )
    .unwrap();

    let result = bank.transfer("Bob", "Alice", Money::from_yuan(300).unwrap());
    assert!(result.is_ok());

    assert_eq!(
        bank.get_balance("Alice").unwrap(),
        Money::from_yuan(800).unwrap()
    );
    assert_eq!(
        bank.get_balance("Bob").unwrap(),
        Money::from_yuan(-200).unwrap()
    );
    assert_eq!(bank.total_assets(), Ok(Money::from_yuan(600).unwrap()));
}
