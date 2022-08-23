mod partida;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// ver https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
// https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

fn main() {
    // leer_palabra();
    let letra = match partida::Partida::obtener_jugada() {
        Ok(letra) => letra,
        Err(_) => return
    };
    println!("Se obtuvo la letra: {}", letra);
}

fn leer_palabra() -> io::Result<()>  {
    // Obtiene un array de argumentos (igual que argv)
    let args: Vec<String> = env::args().collect();
    // Saco el path
    let path = &args[1];
    // Abro el archivo siendo el pwd la raiz del proyecto
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // reader.lines devuelve un iterador pero guarda en memoria
    // cada palabra o linea.
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}