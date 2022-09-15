
use core::time;
use std::{thread::{self}, sync::{Mutex, Arc}};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            loop {
                let num = counter.lock().unwrap();
                if *num > 100 {
                    println!("El numero es mayor a 100!, paro");
                    break;
                } else {
                    println!("Me voy a dormir 1 segundo: zzz");
                    let next_attempt = time::Duration::from_millis(1000);
                    thread::sleep(next_attempt);
                    thread::yield_now();
                }
            }
        });
        handles.push(handle);
    }
    for i in 0..1000 {
        let mut num = counter.lock().unwrap();
        *num = i;
        println!("El numero es {}", num);
        let next_attempt = time::Duration::from_millis(1);
        thread::sleep(next_attempt);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}