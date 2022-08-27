#[allow(unused_imports)]
mod repositorio_lineas;

use std::env;
use std::fs::File;

fn main() {
    let archivo = obtener_archivo().unwrap();
    let mut repositorio_lineas = repositorio_lineas::RepositorioLineas::new(archivo);
    loop {
        let palabra = repositorio_lineas.obtener_linea();
        if palabra.is_empty() {
            break;
        }
        // Hacer algo por ej println! a la linea
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
