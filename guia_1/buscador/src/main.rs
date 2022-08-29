use std::fs;

mod repositorio_terminos;

fn main() {
    let directorio_fuente = fs::read_dir("./documentos").unwrap();
    let mut repositorio_terminos = repositorio_terminos::RepositorioTerminos::new(directorio_fuente);
    repositorio_terminos.obtener_termino();
}