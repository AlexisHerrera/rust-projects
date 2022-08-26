use std::collections::HashMap;

struct ContadorPalabras {
    registros: HashMap<String, i32>
}

impl ContadorPalabras {
    fn new() -> ContadorPalabras {
        ContadorPalabras { registros: HashMap::new() }
    }
    
    fn registrar_palabra(&mut self, palabra: String) {
        self.registros.insert(palabra, 1);
    }
    fn repeticiones_de(&self, palabra: String) -> i32 {
        self.registros.get(&palabra).cloned().unwrap()
    }
    fn esta_vacio(&self) -> bool {
        self.registros.is_empty()
    }

}
#[cfg(test)]
mod tests_registros {
    use super::*;
    #[test]
    fn el_registro_de_palabras_esta_vacio_al_iniciar() {
        let contador_palabras = ContadorPalabras::new();
        assert!(contador_palabras.esta_vacio());
    }

    #[test]
    fn registra_una_palabra_ingresada_nueva() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("felicidad".to_string());
        assert_eq!(contador_palabras.repeticiones_de("felicidad".to_string()), 1);
    }
}
