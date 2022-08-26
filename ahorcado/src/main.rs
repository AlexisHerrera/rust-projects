mod partida;
mod repositorio_palabras;

use std::fs::File;
use std::env;
// ver https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
// https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

fn main() {
    let archivo = obtener_archivo().unwrap();
    let mut repositorio_palabras = repositorio_palabras::RepositorioPalabras::new(archivo);
    loop {
        let palabra = repositorio_palabras.obtener_palabra();
        if palabra == "" {break;}

        let mut partida = partida::Partida::new(palabra, 5);
        partida.iniciar_partida();
    }
}

fn obtener_archivo() -> Result<File, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    // Saco el path
    let path = &args[1];
    // Abro el archivo siendo el pwd la raiz del proyecto
    File::open(path)
}


// nit: Se podria crear un test para que testee que se lee cada palabra?
