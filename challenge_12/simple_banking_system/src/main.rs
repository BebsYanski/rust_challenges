use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    age: u8,
    tel: u128,
}

#[derive(Debug, Clone)]
struct BankAccount {
    id: u32,
    owner: User,
    balance: f64,
}

#[derive(Debug, Clone)]
struct Bank {
    name: String,
    id: u32,
    accounts: HashMap<u32, BankAccount>,
}

fn main() {
    let mut bank: Bank = Bank::new(String::from("Ecobank"));
    println!("The Simple Banking System");
}
fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let input = read_line(prompt);
        match input.parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, Try again."),
        }
    }
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_line(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid amount, Try again."),
        }
    }
}

impl BankAccount {
    fn create_account(owner: User, id: u32) -> Self {
        // TODO: Add a new user with a bank account + initial balance

        BankAccount {
            balance: 0.0,
            id,
            owner,
        }
    }

    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        // TODO: Increase the balance of a specific account
        // todo!("Deposit into Accounts");
        if amount <= 0.0 {
            return Err("Deposit amount must be positive.".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        // TODO: Decrease balance (but only if the have enough money)
        // todo!("Withdraw from Accounts");
        if amount <= 0.0 {
            return Err(String::from("Withdrawal amount must be positive"));
        }
        if amount > self.balance {
            return Err(String::from("Insufficient funds"));
        }
        self.balance -= amount;
        Ok(())
    }
    fn check_balance(self) -> f64 {
        // TODO: View the current status of an account
        return self.balance;
    }
}

impl Bank {
    fn new(name: String) -> Self {
        Bank {
            name,
            id: 1001,
            accounts: HashMap::new(),
        }
    }

    fn create_account(&mut self, owner: User) -> u32 {
        let id = self.id;
        let account = BankAccount::create_account(owner, id);
        self.accounts.insert(id, account);
        self.id += 1;
        id
    }

    fn get_account(&self, id: u32) -> Option<&BankAccount> {
        self.accounts.get(&id)
    }

    fn get_account_mut(&mut self, id: u32) -> Option<&mut BankAccount> {
        self.accounts.get_mut(&id)
    }
}
