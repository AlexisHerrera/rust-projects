use repositorio_terminos::*;

fn main() {
    let terminos = RepositorioTerminos::new("./documentos")
                                            .obtener_terminos()
                                            .expect("No se pudo leer alguno de los archivos");
    println!("{:?}", terminos);
}