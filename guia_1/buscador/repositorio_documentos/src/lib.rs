use std::{fs::{ReadDir, File, DirEntry}};

const STOP_WORDS: &[&str] = &["la", "las", "lo", "los", "el"];
pub struct RepositorioDocumentos {
    directorio_fuente : ReadDir
}

#[derive(Debug)]
pub struct Documento {
    pub nombre: String,
    pub terminos: Vec<String>
}

impl Documento {
    pub fn new(nombre: String, terminos: Vec<String>) -> Documento {
        Documento {
            nombre,
            terminos
        }
    }
}

pub enum DocumentoError {
    NoHayDocumentos,
    ErrorDeLectura
}

impl From<std::io::Error> for DocumentoError {
    fn from(error: std::io::Error) -> Self {
        DocumentoError::ErrorDeLectura
    }
}

impl RepositorioDocumentos {
    pub fn new(dir_path: &str) -> RepositorioDocumentos {
        let directorio_fuente = obtener_directorio(dir_path).unwrap();
        RepositorioDocumentos {
            directorio_fuente : directorio_fuente
        }
    }
    // Decidí que solo se puede obtener TODOS los terminos o ninguno,
    // porque ir iterando sobre cada archivo además de overkill no tiene
    // mucho sentido ya que siempre voy a querer el contenido de TODOS los 
    // archivos. Eso si, asumo que los archivos pueden no entrar en memoria.
    pub fn obtener_terminos(&mut self) -> Result<Vec<String>, std::io::Error> {
        let mut terminos : Vec<String> = vec![];
        loop {
            let file_dir = match self.directorio_fuente.next() {
                Some(f) => f,
                None => {break;}
            };
            let f = obtener_archivo(&file_dir?)?;
            let terminos_archivo = obtener_terminos_de_archivo(f);
            for termino in terminos_archivo {
                terminos.push(termino);
            }
        }
        Ok(terminos)
    }

    pub fn obtener_documento(&mut self) -> Result<Documento, self::DocumentoError> {
        let file_dir = match self.directorio_fuente.next() {
            Some(f) => f,
            None => {return Err(self::DocumentoError::NoHayDocumentos)}
        };
        let dir_entry = file_dir?;

        let f = obtener_archivo(&dir_entry)?;
        let terminos = obtener_terminos_de_archivo(f);
        
        let nombre_archivo = match obtener_nombre_archivo(&dir_entry){
            Ok(x) => x,
            Err(_) => {return Err(self::DocumentoError::ErrorDeLectura)}
        };
        Ok(Documento{nombre: nombre_archivo, terminos: terminos})
    }
}

// Estos serían metodos privados, la verdad no se donde meterlos

fn obtener_directorio(dir_path: &str) -> Result<ReadDir, std::io::Error> {
    std::fs::read_dir(dir_path)
}

/// Recibe un DirEntry (es lo que se obtiene al iterar un ReadDir)
fn obtener_archivo(file_dir: &DirEntry) -> Result<File, std::io::Error> {
    File::open(file_dir.path().as_path())
}

fn obtener_nombre_archivo(dir_entry : &DirEntry) -> Result<String, std::io::Error> {
    match dir_entry.file_name().into_string() {
        Ok(x) => Ok(x),
        Err(_) => {return Err(std::io::Error::new(std::io::ErrorKind::Other, "No se ha podido acceder al archivo"))}
    }
}

use repositorio_lineas::*;
fn obtener_terminos_de_archivo(file: File) -> Vec<String> {
    let mut repositorio_lineas = RepositorioLineas::new(file);
    let mut terminos = Vec::new();
    loop {
        let linea = repositorio_lineas.obtener_linea();
        if linea.is_empty() { break;}
        let terminos_linea = linea_a_terminos(&linea);
        for termino in terminos_linea {
            terminos.push(termino);
        }
    }
    terminos
}


pub fn linea_a_terminos(linea: &str) -> Vec<String> {
    let mut terminos : Vec<String> = vec![];
    for palabra in linea.split(' ') {
        if STOP_WORDS.contains(&palabra) {
            continue;
        }
        terminos.push(palabra.to_string().to_lowercase());
    }
    terminos
}


#[cfg(test)]
mod test_linea_a_terminos {
    use super::linea_a_terminos;
    
    #[test]
    fn una_linea_con_solo_una_palabra_devuelve_un_solo_termino() {
        let linea = "hola";

        assert_eq!(linea_a_terminos(linea), vec!["hola".to_string()]);
    }

    #[test]
    fn una_linea_con_2_palabras_devuelve_un_vector_de_palabras() {
        let linea = "hola como";

        assert_eq!(linea_a_terminos(linea), vec!["hola".to_string(), "como".to_string()]);
    }

    #[test]
    fn una_linea_con_palabras_en_distinto_case_los_lleva_a_downcase() {
        let linea = "Hola Como";

        assert_eq!(linea_a_terminos(linea), vec!["hola".to_string(), "como".to_string()]);
    }

    #[test]
    fn una_linea_que_tiene_stop_words_se_los_elimina() {
        let linea = "el las lo los dinosaurio";

        assert_eq!(linea_a_terminos(linea), vec!["dinosaurio".to_string()]);

    }
}