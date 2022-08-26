use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct RepositorioPalabras {
    reader: BufReader<File>,
}

impl RepositorioPalabras {
    pub fn new(inner: File) -> RepositorioPalabras {
        RepositorioPalabras {
            reader: BufReader::new(inner),
        }
    }

    pub fn obtener_palabra(&mut self) -> String {
        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .expect("Error al leer la linea del archivo");
        line.trim().to_string()
    }
}
