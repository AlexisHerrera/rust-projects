use repositorio_documentos::*;
mod buscador;

fn main() {
    // let terminos = RepositorioDocumentos::new("./documentos")
    //                                         .obtener_terminos()
    //                                         .expect("No se pudo leer alguno de los archivos");
    // println!("{:?}", terminos);

    let mut repo = RepositorioDocumentos::new("./documentos");
    let mut documentos = vec![];
    loop {
        let documento = match repo.obtener_documento() {
            Ok(documento) => documento,
            Err(_error) => {break;}
        };
        documentos.push(documento);
    }
    let mut nombres_documentos = vec![];
    let mut terminos: Vec<String> = vec![];
    for mut documento in documentos {
        nombres_documentos.push(documento.nombre);
        terminos.append(&mut documento.terminos);
    }

    println!("Nombres de documentos: ");
    println!("{}", nombres_documentos.join(", "));
    println!("Terminos de documentos: ");
    println!("{}", terminos.join(", "));
}