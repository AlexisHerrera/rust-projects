use std::io::{Error, BufRead, Write};

pub fn upcase(
    input: &mut impl BufRead,
    output: &mut impl Write,
) -> Result<(), Error> {
    let mut buffer = "".to_string();

    input.read_line(&mut buffer)?;
    output.write_all(buffer.to_uppercase().as_bytes())?;

    Ok(())
}

// use std::io::{Error, Read, Write};
use std::io::{Read};
pub fn upcase2(
    input: &mut impl Read,
    output: &mut impl Write,
) -> Result<(), Error> {
    let mut buffer = "".to_string();

    input.read_to_string(&mut buffer)?;
    output.write_all(buffer.to_uppercase().as_bytes())?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_upcased_input_to_output() {
        let mut output: Vec<u8> = Vec::new();

        upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
        assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}