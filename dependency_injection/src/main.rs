use std::io::{self, BufReader};
mod lib;
use lib::upcase2;
use lib::upcase;

// Usa BufReader para el input, ya que usa get_line
fn main() -> io::Result<()> {
    upcase(&mut BufReader::new(std::io::stdin()), &mut io::stdout())
}

// Lee desde el stdin, pero no se detiene porque usa read_to_string() (que espera el EOF)
fn main2() -> io::Result<()> {
    upcase2(&mut io::stdin(), &mut io::stdout())
}