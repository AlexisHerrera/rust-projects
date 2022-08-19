use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
    
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // An associated function is a function that’s implemented on a type, in this case String
        let mut guess = String::new();
        // if this instance of Result is an Err value, expect will cause the program to crash and display the
        // message that you passed as an argument to expect. Why needs the reference needs to be mutable for read_line?
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");
        // Extra
        // println!("Bytes read: {bytes_read}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}

