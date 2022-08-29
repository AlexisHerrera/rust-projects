use std::{fs::{ReadDir, File}, io::Read};

pub struct RepositorioTerminos {
    directorio_fuente: ReadDir,
}

impl RepositorioTerminos {
    pub fn new(directorio_fuente: ReadDir) -> RepositorioTerminos{
        RepositorioTerminos {
            directorio_fuente
        }
    }

    pub fn obtener_termino(&mut self) -> String {
        let file_dir = self.directorio_fuente.next().unwrap();
        let mut f = File::open(file_dir.unwrap().path().as_path()).unwrap();
        // let mut repositorio_lineas = repositorio_lineas::RepositorioLineas::new(f);
        "xd".to_string()
    }
}
