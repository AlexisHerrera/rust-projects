fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
    let fahrenheit_temperature :f64 = 80.0;
    println!("{} °F is {}°C", fahrenheit_temperature, fahrenheit_to_celsius(fahrenheit_temperature));
    let celsius_temperature :f64 = 26.666666666666666666666666;
    println!("{} °C is {}°F", celsius_temperature, celsius_to_fahrenheit(celsius_temperature));

}

fn example1() {
    loop {
        println!("again!");
        break;
    }
}

fn example2() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn example3() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // (!) i didn't know this was possible
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


fn example4() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn example5() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn example6() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn example7() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    5.0/9.0 * (fahrenheit - 32.0)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius*1.8 + 32.0
}

fn get_nth_fibonacci_number(n : i32) -> i64 {
    1000*n
}