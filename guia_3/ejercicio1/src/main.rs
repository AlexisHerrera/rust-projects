//! Voy a suponer que se quiere que se quiere que el balance de 0.
//! Por ejemplo posibles alternativas de ejecuciones:
//! - +40 -30 +60 -70
//! - +40 +60 -30 -70
//! - +60 +40 -30 -70
//! - +60 +40 -70 -30
//! etc, siempre que el balance parcial sea mayor a 0.
use std::{thread, sync::{Arc, RwLock}};

struct Account(i32);

impl Account {
    fn deposit(&mut self, amount: i32) {
        println!("op: deposit {}, available funds: {:?}", amount, self.0);
        self.0 += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        println!("op: withdraw {}, available funds: {}", amount, self.0);
        if self.0 >= amount {
            self.0 -= amount;
        } else {
            panic!("Error: Insufficient funds.")
        }
    }

    fn balance(&self) -> i32 {
        self.0
    }
}

fn main() {
    let account: Account = Account(0);
    let lock = Arc::new(RwLock::new(account));

    let lock1 = lock.clone();
    let customer1_handle = thread::spawn(move || {
        if let Ok(mut account) = lock1.write() {
            account.deposit(40);
        }
    });

    let lock2 = lock.clone();
    let customer2_handle = thread::spawn(move || {
        if let Ok(mut account) = lock2.write() {
            if account.balance() >= 30 {
                account.withdraw(30);
            }
        }
    });

    let lock3 = lock.clone();
    let customer3_handle = thread::spawn(move || {
        if let Ok(mut account) = lock3.write() {
            account.deposit(60);
        }
    });

    let lock4 = lock.clone();
    let customer4_handle = thread::spawn(move || {
        if let Ok(mut account) = lock4.write() {
            if account.balance() >= 70 {
                account.withdraw(70);
            }
        }
    });

    let handles = vec![
    customer1_handle,
    customer2_handle,
    customer3_handle,
    customer4_handle,
    ];

    for handle in handles {
        handle.join().unwrap();
    }

    let savings = lock.read().unwrap().balance();

    println!("Balance: {:?}", savings);
}