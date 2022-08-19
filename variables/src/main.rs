const SPEED_OF_LIGHT:u64 = 299_792_458;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // How to format to 299.792,458 ? 
    println!("Speed of light is: {SPEED_OF_LIGHT} m/s !");
    
    // Other excersise (shadowing)

    let x = 5;
    // Note that x is not mutable
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("Se imprime spaces: {spaces}");
    let spaces = spaces.len();
    println!("Se imprime spaces: {spaces}");

    /* let mut spaces = "   ";
    spaces = spaces.len(); */
    numeric_operations();
    tuples_operations();
    array_type();
}

fn numeric_operations() {
    
    // Numeric operations


    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;
}

fn tuples_operations() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is : {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
              println!("{:?}", months);
    let b = [3; 5];
    println!("{:?}", b);
    let first = a[0];
    println!("a[0]: {first}");
    
    println!("Unreacheable - Runtime Error");
    a[100];
}