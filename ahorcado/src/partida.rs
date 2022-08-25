use std::io::BufRead;

#[derive(Debug)]

pub enum JugadaError {
    Lectura,
    Formato,
}

pub struct Partida {
    intentos: i32,
    letras_descubiertas: Vec<char>,
    palabra_secreta: String
}

impl Partida {

    // ver si se puede cambiar a str
    fn new(palabra_secreta: String, intentos: i32) -> Partida {
        Partida {
            palabra_secreta,
            intentos,
            letras_descubiertas: vec![]
        }
    }
    // Resta un intento
    fn reducir_intentos(&mut self) {
        self.intentos -= 1;
    }

    // Devuelve true si quedan intentos, false en caso contrario
    fn quedan_intentos(&self) -> bool {
        self.intentos > 0
    }

    // Se testea con dependency injection sobre el input: https://jeffkreeftmeijer.com/rust-stdin-stdout-testing/
    // El metodo obtener_letra se encarga de parsear el input
    pub fn obtener_jugada(input: &mut impl BufRead) -> Result<char, JugadaError> {
        let mut input_string = String::new();
        // Cambiar a unwrap_or
        match input.read_line(&mut input_string) {
            Ok(_) => (),
            Err(_) => return Err(JugadaError::Lectura),
        }
        let input_string = input_string.trim();
        println!("Largo de input_string es {}", input_string.len());
        Partida::obtener_letra(input_string)
    }

    // Asume que recibio un string, lo parsea
    fn obtener_letra(input_str: &str) -> Result<char, JugadaError> {
        if input_str.len() != 1 {
            return Err(JugadaError::Formato);
        }
        let letra = input_str.chars().next().unwrap();
        if !letra.is_alphabetic() {
            return Err(JugadaError::Formato);
        }
        Ok(letra)
    }

    fn realizar_jugada(&mut self, letra: char) {
        // if self.letras_descubiertas.contains(&letra) {

        // }
        for letra_secreta in self.palabra_secreta.chars() {
            if letra_secreta == letra {
                self.letras_descubiertas.push(letra);
                return ;
            }
        }
        self.reducir_intentos();
    }
}

#[cfg(test)]
mod tests {
    use super::Partida;

    #[test]
    fn disminuye_intentos() {
        let mut partida = Partida::new("hola".to_string(), 5);
        partida.reducir_intentos();
        assert_eq!(partida.intentos, 4);
    }

    #[test]
    fn quedan_intentos() {
        let mut partida = Partida::new("hola".to_string(), 1);
        partida.reducir_intentos();
        assert!(!partida.quedan_intentos());
    }
    #[test]
    fn realizar_jugada_que_descubre_letra() {
        let mut partida = Partida::new("camion".to_string(), 5);
        partida.realizar_jugada('c');
        assert_eq!(partida.letras_descubiertas, vec!['c']);
        assert_eq!(partida.intentos, 5);
    }

    #[test]
    fn realizar_jugada_que_no_descubre_letra() {
        let mut partida = Partida::new("camion".to_string(), 5);
        partida.realizar_jugada('f');
        assert_eq!(partida.letras_descubiertas, vec![]);
        assert_eq!(partida.intentos, 4);
    }

}
// Estos testean sobre el metodo obtener_jugada, es más de integración.
mod test_input_jugada {
    use super::Partida;
    #[test]
    fn obtiene_una_jugada() {
        let letra = Partida::obtener_jugada(&mut "c\n".as_bytes()).unwrap();
        assert_eq!(letra, 'c');
    }
}

mod test_parse_string {
    use super::Partida;
    
    #[test]
    fn se_obtiene_la_letra_ingresada() {
        assert_eq!(Partida::obtener_letra("c").unwrap(), 'c');
    }
    #[test]
    fn verifica_que_sea_una_sola_letra() {
        assert!(Partida::obtener_letra("camisa").is_err());
    }

    #[test]
    fn se_obtiene_error_si_el_input_es_vacio() {
        assert!(Partida::obtener_letra("").is_err());
    }
    #[test]
    fn se_obtiene_error_jugada_si_no_es_una_letra() {
        assert!(Partida::obtener_letra("1").is_err());
    }
}
