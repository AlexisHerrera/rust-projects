use repositorio_documentos::*;

mod buscador;

fn main() {
    let repo = RepositorioDocumentos::new("./documentos");
    let mut buscador = buscador::Buscador::new();
    buscador.cargar_documentos(repo);
    println!("{:#?}", buscador);
}