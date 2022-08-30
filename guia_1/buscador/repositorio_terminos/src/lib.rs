use std::{fs::{ReadDir, File}};

pub struct RepositorioTerminos {
}

impl RepositorioTerminos {
    pub fn new() -> RepositorioTerminos {
        RepositorioTerminos {}
    }
    // Sacar los unwraps por dios!
    // Decidí que solo se puede obtener TODOS los terminos o ninguno,
    // porque ir iterando sobre cada archivo además de overkill no tiene
    // mucho sentido ya que siempre voy a querer el contenido de TODOS los 
    // archivos
    pub fn obtener_terminos(&self, directorio_fuente: ReadDir) -> Vec<String> {
        let mut terminos : Vec<String> = vec![];
        for file_dir in directorio_fuente {
            // Falta sacar el unwrap de file_dir
            let f = File::open(file_dir.unwrap().path().as_path()).unwrap();
            let mut repositorio_lineas = repositorio_lineas::RepositorioLineas::new(f);
            loop {
                // Falta pasarlo a terminos: ie: una sola palabra
                let linea = repositorio_lineas.obtener_linea();
                if linea.is_empty() { break;}
                terminos.push(linea);   
            }
        }
        terminos
    }
}
