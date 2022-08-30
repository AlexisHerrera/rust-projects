use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct RepositorioLineas {
    reader: BufReader<File>,
}

impl RepositorioLineas {
    pub fn new(inner: File) -> RepositorioLineas {
        RepositorioLineas {
            reader: BufReader::new(inner),
        }
    }

    // Devuelve una linea sin el salto de linea
    pub fn obtener_linea(&mut self) -> String {
        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .expect("Error al leer la linea del archivo");
        line.trim().to_string()
    }
}
