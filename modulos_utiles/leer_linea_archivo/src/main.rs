#[allow(unused_imports)]
mod repositorio_palabras;

use std::env;
use std::fs::File;

fn main() {
    let archivo = obtener_archivo().unwrap();
    let mut repositorio_palabras = repositorio_palabras::RepositorioPalabras::new(archivo);
    loop {
        let palabra = repositorio_palabras.obtener_palabra();
        if palabra.is_empty() {
            break;
        }
        // Hacer algo por ej println! a la palabra
        println!("{}", palabra);
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
