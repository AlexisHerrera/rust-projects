fn main() {
    example1(); // functions can take ownership
    example2(); // functions can give ownership, another example of move and copy
    example3(); // Hard way to borrow
    example4(); // Borrowing (references)
    example5(); // Mutable references
    example6(); // mutable ref + inmutable life cycle
    example7(); // No dangling pointers
    example8();
}

fn example8() {
    let mut full_string = String::from("Hello world");
    let first_word = get_first_word(&full_string);
    println!("First word is:{}", first_word);
    full_string.clear();
    // Lo siguiente no funciona porque no se puede prestar como mutable
    // y inmutable al mismo tiempo: ver get_first_word(&full_string) y full_string.clear() == clear(&mut full_string)
    // Si lo comento la deja pasar porque como la mutable nadie la vuelve a leer no hay problema.
    // println!("First word is:{}", first_word);
}

fn get_first_word(full_string: &String) -> &str {
    let bytes = full_string.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &full_string[0..i];
        }
    }
    &full_string[..]
}

fn example7() {
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     // this wont work because s is going to be dropped when it gets out of scope
//     s;
// }

fn no_dangle() -> String { // This will work because is giving ownership to the caller.
    let s = String::from("hello");
    s
}

fn example6() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // The next line will make not compile
    // println!("{}", r1);
}

fn example5() {
    let mut s = String::from("hello");
    // let x = &mut s;
    // let y = &s;
    // println!("x:{}, y:{}", x, y);
    change(&mut s);
}

// fn not_working() {
//     let x = &mut s;
//     let y = &s;
//     println!("x:{}, y:{}", x, y);
//     let x = &mut s;
//     let y = &mut s;
//     println!("x:{}, y:{}", x, y);
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn example4() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn example3() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length2(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length2(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


fn example2() {
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    // let s4 = gives_ownership();

    // println!("s1:{} and s4: {}", s1, s4);
    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// example 1

fn example1() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
                        // println!("{}", s); This will not work because s was moved to takes_ownership function
    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
