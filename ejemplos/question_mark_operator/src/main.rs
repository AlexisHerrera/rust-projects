use std::num::ParseIntError;

fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}

fn main() {
    let res = try_to_parse();
    println!("{:?}", res);
}


use std::fs::File;
use std::io;

#[derive(Debug)]
struct AppError {
    kind: String,    // type of the error
    message: String, // error message
}

// Implement std::convert::From for AppError; from io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

fn main2() -> Result<(), AppError> {
    let _file = File::open("nonexistent_file.txt")?; // This generates an io::Error. But because of return type is Result<(), AppError>, it converts to AppError

    Ok(())
}
