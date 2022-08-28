use std::collections::HashMap;
// TODO: Hacer un struct para palabra, sería lo correcto
#[derive(Debug, PartialEq)]
struct Registro {
    palabra: String,
    apariciones: i32
}
impl Registro {
    fn new(palabra: String, apariciones: i32) -> Registro {
        Registro {
            palabra,
            apariciones
        }
    }
}
pub struct ContadorPalabras {
    registros: HashMap<String, i32>
}
impl ContadorPalabras {
    pub fn new() -> ContadorPalabras {
        ContadorPalabras { registros: HashMap::new() }
    }
 
    /// Asume que palabra es una palabra no una frase. Es decir que ya está parseada
    fn registrar_palabra(&mut self, palabra: String) {
        let palabra = palabra.to_lowercase();
        let cantidad_apariciones = match self.registros.get(&palabra) {
            Some(cantidad_apariciones) => *cantidad_apariciones,
            None => 0
        };
        self.registros.insert(palabra, cantidad_apariciones+1);
    }
    
    #[cfg(test)]
    fn repeticiones_de(&self, palabra: String) -> i32 {
        let palabra = palabra.to_lowercase();
        *self.registros.get(&palabra).unwrap()
    }
    
    #[cfg(test)]
    fn esta_vacio(&self) -> bool {
        self.registros.is_empty()
    }

    // Esto se puede cambiar a -> Vec<(&String, &i32)>
    // Hay un problema, esta copiando de nuevo los registros, y esto en teoria no es necesario
    // Solo haría falta un borrow porque no va a modificar los datos.
    fn obtener_ranking_registros(&self) -> Vec<Registro> {
        let mut registros: Vec<Registro> = vec![];
        for (palabra, apariciones) in &self.registros {
            registros.push(Registro::new((*palabra).clone(), *apariciones));
        }
        registros.sort_by(|a, b| b.apariciones.cmp(&a.apariciones));
        registros
    }

    pub fn registrar_texto(&mut self, texto: String) {
        let parser = TextoAPalabras::new();
        let palabras = parser.texto_a_palabras(texto);
        for palabra in palabras {
            self.registrar_palabra(palabra);
        }
    }

    pub fn imprimir_ranking(&self) {
        let ranking_registros = self.obtener_ranking_registros();
        for registro in ranking_registros {
            println!("{} -> {}", registro.palabra, registro.apariciones);
        }
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
    fn registrar_correctamente_un_texto() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_texto("felicidad felicidad tristeza".to_string());
        assert_eq!(contador_palabras.repeticiones_de("felicidad".to_string()), 2);
        assert_eq!(contador_palabras.repeticiones_de("tristeza".to_string()), 1);
    }

    #[test]
    fn registrar_es_case_insensitive() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("FeliciDad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        assert_eq!(contador_palabras.repeticiones_de("felicidad".to_string()), 2);
    }

    #[test]
    fn obtener_registro_es_case_insensitive() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        assert_eq!(contador_palabras.repeticiones_de("FELICIDAD".to_string()), 2);
    }
}

mod test_ranking {
    use super::*;
    #[test]
    fn ranking_de_registros_devuelve_los_registros_ordenados() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("tristeza".to_string());

        let registro_felicidad = Registro::new("felicidad".to_string(), 2);
        let registro_tristeza = Registro::new("tristeza".to_string(), 1);
        assert_eq!(contador_palabras.obtener_ranking_registros(), vec![registro_felicidad, registro_tristeza]);
    }

    #[test]
    fn ranking_de_registros_esta_vacio_si_no_hay_registros() {
        let mut contador_palabras = ContadorPalabras::new();
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("felicidad".to_string());
        contador_palabras.registrar_palabra("tristeza".to_string());
        contador_palabras.registrar_palabra("tristeza".to_string());
        contador_palabras.registrar_palabra("calma".to_string());

        let registro_calma = Registro::new("calma".to_string(), 1);
        let registro_tristeza = Registro::new("tristeza".to_string(), 2);
        let registro_felicidad = Registro::new("felicidad".to_string(), 3);
        assert_eq!(contador_palabras.obtener_ranking_registros(), vec![registro_felicidad, registro_tristeza, registro_calma]);
    }

    #[test]
    fn ranking_devuelve_ordenado_con_multiples_registros() {
        let contador_palabras = ContadorPalabras::new();
        assert!(contador_palabras.obtener_ranking_registros().is_empty());
    }
}

// Se podría pasar a otro archivo? Debería hacer test de integración
struct TextoAPalabras;

impl TextoAPalabras {
    fn new() -> Self {
        TextoAPalabras {}
    }

    // Devuelve un arreglo de palabras dado una linea de texto de palabras separadas por espacio
    fn texto_a_palabras(&self, texto: String) -> Vec<String> {
        let texto_trimeado = texto.trim().to_string();
        if texto_trimeado.is_empty() { return vec![]; }
        let mut palabras = vec![];
        for palabra in texto_trimeado.split(' ') {
            palabras.push(palabra.to_string())
        }
        palabras
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;
    #[test]
    fn una_linea_con_una_palabra_solo_devuelve_la_palabra() {
        let parser = TextoAPalabras::new();
        let texto = "hola\n".to_string();
        assert_eq!(parser.texto_a_palabras(texto), vec!["hola".to_string()]);
    }
    #[test]
    fn una_linea_con_multiples_palabras_devuelve_las_palabras() {
        let parser = TextoAPalabras::new();
        let texto = "hola como estas\n".to_string();
        assert_eq!(parser.texto_a_palabras(texto), vec!["hola".to_string(), "como".to_string(), "estas".to_string()]);
    }

    #[test]
    fn una_linea_vacia_no_la_traduce_a_palabra() {
        let parser = TextoAPalabras::new();
        let texto = "".to_string();
        assert!(parser.texto_a_palabras(texto).is_empty());
    }
}