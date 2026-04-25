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
        BankAccount {
            balance: 0.0,
            id,
            owner,
        }
    }

    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        // todo!("Deposit into Accounts");
        if amount <= 0.0 {
            return Err("Deposit amount must be positive.".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
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

    fn transfer(&mut self, from: u32, to: u32, amount: f64) -> Result<(), String> {
        if from == to {
            return Err("Cannot transfer to the same account.".to_string());
        }
        if amount <= 0.0 {
            return Err(String::from("Transfer amount must be positive."));
        }

        // Check funds first to avoid partial updates
        {
            let from_acc = self
                .accounts
                .get(&from)
                .ok_or("Source account not found".to_string())?;

            if from_acc.balance < amount {
                return Err(String::from("Insufficient funds for transfer."));
            }
        }

        // Ensure target exists before mutating
        if !self.accounts.contains_key(&to) {
            return Err("Destination account not found.".to_string());
        }

        // Perform Transfer
        self.accounts.get_mut(&from).unwrap().balance -= amount;
        self.accounts.get_mut(&to).unwrap().balance += amount;
        Ok(())
    }
}
