use std::fs;

use repositorio_terminos::*;
fn main() {
    let directorio_fuente = fs::read_dir("./documentos").unwrap();
    let terminos = RepositorioTerminos::new().obtener_terminos(directorio_fuente);
    println!("terminos: {:?}", terminos);
}