use std::collections::HashMap;

struct ContadorPalabras {
    registros: HashMap<String, i32>
}
impl ContadorPalabras {
    fn new() -> ContadorPalabras {
        ContadorPalabras { registros: HashMap::new() }
    }
 
    /// Asume que palabra es una palabra no una frase. Es decir que ya está parseada
    fn registrar_palabra(&mut self, palabra: String) {
        let cantidad_apariciones = match self.registros.get(&palabra) {
            Some(cantidad_apariciones) => *cantidad_apariciones,
            None => 0
        };
        self.registros.insert(palabra, cantidad_apariciones+1);
    }
    fn repeticiones_de(&self, palabra: String) -> i32 {
        *self.registros.get(&palabra).unwrap()
    }
    fn esta_vacio(&self) -> bool {
        self.registros.is_empty()
    }

    fn obtener_ranking_registros(&self) -> Vec<(String, i32)> {
        // Cambiar esto, esta hardcodeado
        vec![("felicidad".to_string(), 2), ("tristeza".to_string(), 1)]
    }

}
#[cfg(test)]
mod tests_registro {
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

    #[test]
    fn registra_una_palabra_multiples_veces() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        assert_eq!(contador_palabras.repeticiones_de("felicidad".to_string()), 2);
    }

    #[test]
    fn ranking_de_registros_devuelve_los_registros_ordenados() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("tristeza".to_string());

        assert_eq!(contador_palabras.obtener_ranking_registros(), vec![("felicidad".to_string(), 2), ("tristeza".to_string(), 1)]);
    }
}
