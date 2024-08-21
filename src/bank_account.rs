#[derive(Debug)]
enum AccountType {
    Current,
    Savings,
}

enum AccountError {
    InsufficientFunds,
}

struct Account {
    account_number: u32,
    balance: f64,
    account_type: AccountType,
}

impl Account {
    fn new(account_number: u32, account_type: AccountType) -> Account {
        Account {
            account_number,
            balance: 0.0,
            account_type,
        }
    }

    fn display(&self) {
        println!(
            "Account Number: {}, Balance: {:.2}, Type: {:?}",
            self.account_number, self.balance, self.account_type
        );
    }
}

trait AccountOperations {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), AccountError>;
    fn transfer(&mut self, other: &mut Account, amount: f64) -> Result<(), AccountError>;
}

impl AccountOperations for Account {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), AccountError> {
        if self.balance < amount {
            return Err(AccountError::InsufficientFunds);
        }
        self.balance -= amount;
        Ok(())
    }

    fn transfer(&mut self, other: &mut Account, amount: f64) -> Result<(), AccountError> {
        self.withdraw(amount)?;
        other.deposit(amount);
        Ok(())
    }
}

pub fn run_banck_account() {
    println!("------- bank account -------");
    let mut account1 = Account::new(1234, AccountType::Current);
    let mut account2 = Account::new(456, AccountType::Savings);

    account1.deposit(100.0);
    account1.display();
    account2.display();

    match account1.withdraw(50.0) {
        Ok(()) => println!("Withdrawal successful!"),
        Err(AccountError::InsufficientFunds) => println!("Not enough funds."),
    }

    match account1.transfer(&mut account2, 51.0) {
        Ok(()) => println!("Transfer successful!"),
        Err(AccountError::InsufficientFunds) => println!("Not enough funds."),
    }

    account1.display();
    account2.display();

    println!("\n");
    println!("\n");
}
