use std::sync::mpsc;
use std::{thread, vec};
use std::time::Duration;

fn main() {
    // example3();
    example5();
}

#[allow(dead_code)]
fn example1() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handler.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[allow(dead_code)]
fn example2() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });
    // this won't compile because v was moved to the thread
    // drop(v);
    handle.join().unwrap();
}

#[allow(dead_code)]
fn example3() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        //thread::sleep(Duration::from_secs(3));
        tx.send(val).unwrap();
        // this won't compile
        // println!("val is {}", val);
    });

    //println!("Waiting for transmitter ...");
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[allow(dead_code)]
fn example4() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[allow(dead_code)]
fn example5() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

}