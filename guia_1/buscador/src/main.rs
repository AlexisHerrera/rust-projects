use std::io;

use repositorio_documentos::*;

mod buscador;

fn main() {
    let repo = RepositorioDocumentos::new("./documentos");
    let mut buscador = buscador::Buscador::new();
    buscador.cargar_documentos(repo);
    let mut input_busqueda = String::new();
    println!("Ingrese los terminos a buscar: ");
    io::stdin()
        .read_line(&mut input_busqueda)
        .expect("Error al leer la busqueda");
    let resultado = buscador.realizar_busqueda(&input_busqueda);
    for documento_puntaje in resultado {
        println!(
            "{} -> {}",
            documento_puntaje.nombre_documento, documento_puntaje.puntaje
        );
    }
}
