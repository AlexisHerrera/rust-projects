#[allow(unused_imports)]
mod contador_palabras;
#[allow(unused_imports)]
mod repositorio_lineas;

use std::env;
use std::fs::File;

// Lo saco del proyecto obtener_lineas_archivos.
// Tal vez se pueda utilizar como crate externo
fn main() {
    let archivo = obtener_archivo().unwrap();
    let mut repositorio_lineas = repositorio_lineas::RepositorioLineas::new(archivo);
    let mut contador_palabras = contador_palabras::ContadorPalabras::new();
    loop {
        let linea = repositorio_lineas.obtener_linea();
        if linea.is_empty() {
            break;
        }
        contador_palabras.registrar_texto(linea);
    }
    contador_palabras.imprimir_ranking();
}
// Se lee el archivo indicado por el primer argumento al correr la aplicacion
fn obtener_archivo() -> Result<File, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    // Saco el path
    let path = &args[1];
    // Abro el archivo siendo el pwd la raiz del proyecto
    File::open(path)
}
