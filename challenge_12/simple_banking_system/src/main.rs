use std::collections::HashMap;

struct User {
    id: u32,
    name: String,
    age: u8,
    tel: u128,
}
struct BankAccount {
    id: u32,
    owner: User,
    balance: f64,
}

struct Bank {
    bank: HashMap<u32, BankAccount>,
}

fn main() {
    println!("Hello, world!");
}

impl Bank {
    fn create_account() {
        // TODO: Add a new user with a bank account + initial balance
        todo!("Create Accounts");
    }

    fn deposit() {
        // TODO: Increase the balance of a specific account
        todo!("Deposit into Accounts");
    }

    fn withdraw() {
        // TODO: Decrease balance (but only if the have enough money)
        todo!("Withdraw from Accounts");
    }
    fn check_balance() {
        // TODO: View the current status of an account
        todo!()
    }
}
