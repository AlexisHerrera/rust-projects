mod partida;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// ver https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
// https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

fn main() {
    // Por ahora solo inicio el juego con una palabra
    let mut partida = partida::Partida::new("casino".to_string(), 5);
    partida.iniciar_partida();
}

fn obtener_palabra_secreta() -> io::Result<()> {
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
