use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]

pub enum JugadaError {
    Lectura,
    Formato,
}

pub struct Partida {
    intentos: i32,
    letras_descubiertas: Vec<char>,
    letras_sin_exito: Vec<char>,
    palabra_secreta: String,
}

impl Partida {
    // ver si se puede cambiar a str
    pub fn new(palabra_secreta: String, intentos: i32) -> Partida {
        Partida {
            palabra_secreta,
            intentos,
            letras_descubiertas: vec![],
            letras_sin_exito: vec![],
        }
    }
    // Resta un intento
    fn reducir_intentos(&mut self) {
        self.intentos -= 1;
    }

    fn quedan_letras(&self) -> bool {
        for letra_secreta in self.palabra_secreta.chars() {
            if !self.letras_descubiertas.contains(&letra_secreta) {
                return true;
            }
        }
        false
    }

    // Devuelve true si quedan intentos, false en caso contrario
    fn continua_juego(&self) -> bool {
        self.intentos > 0 && self.quedan_letras()
    }

    // Se testea con dependency injection sobre el input: https://jeffkreeftmeijer.com/rust-stdin-stdout-testing/
    // El metodo obtener_letra se encarga de parsear el input
    pub fn obtener_jugada(input: &mut impl Read) -> Result<char, JugadaError> {
        let mut input_string = String::new();
        // Para que pueda leer linea a linea
        let mut input_buf = BufReader::new(input);
        // Cambiar a unwrap_or
        match input_buf.read_line(&mut input_string) {
            Ok(_) => (),
            Err(_) => return Err(JugadaError::Lectura),
        }
        let input_string = input_string.trim();
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
        if self.letras_descubiertas.contains(&letra) {
            return;
        }
        for letra_secreta in self.palabra_secreta.chars() {
            if letra_secreta == letra {
                self.letras_descubiertas.push(letra);
                return;
            }
        }
        self.reducir_intentos();
        self.letras_sin_exito.push(letra);
    }

    fn imprimir_estado_juego(&self) {
        print!("\nLa palabra hasta el momento es: ");
        for letra_secreta in self.palabra_secreta.chars() {
            if self.letras_descubiertas.contains(&letra_secreta) {
                print!("{} ", letra_secreta);
            } else {
                print!("_ ");
            }
        }

        println!();
        print!("Adivinaste las siguientes letras: ");
        for letra_adivinada in self.letras_descubiertas.iter() {
            print!("{} ", letra_adivinada);
        }
        println!();
        print!("Letras que no funcionaron: ");
        for letras_error in self.letras_sin_exito.iter() {
            print!("{} ", letras_error);
        }
        println!();
        println!("Te quedan {} intentos.", self.intentos);
        print!("Ingresa una letra: ");
        io::stdout().flush().unwrap();
    }

    pub fn iniciar_partida(&mut self) {
        // Moverlo al crear la partida?
        let mut input = std::io::stdin();
        println!("Bienvenido al ahorcado de FIUBA!");
        while self.continua_juego() {
            self.imprimir_estado_juego();
            let jugada = Partida::obtener_jugada(&mut input);
            let letra = match jugada {
                Ok(letra) => letra,
                Err(_) => continue,
            };
            self.realizar_jugada(letra);
        }
        self.imprimir_mensaje_final()
    }

    pub fn imprimir_mensaje_final(&self) {
        if self.quedan_letras() {
            // Perdio
            println!("Perdiste!")
        } else {
            println!("Ganaste!")
        }
        println!("La palabra secreta era : {}", self.palabra_secreta)
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
    fn partida_termina_cuando_contador_es_0() {
        let mut partida = Partida::new("hola".to_string(), 1);
        partida.reducir_intentos();
        assert!(!partida.continua_juego());
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
        assert_eq!(partida.letras_sin_exito, vec!['f']);
        assert_eq!(partida.intentos, 4);
    }
    #[test]
    fn realizar_jugada_repetida_que_descubre_no_altera_el_juego() {
        let mut partida = Partida::new("camion".to_string(), 5);
        partida.realizar_jugada('c');
        partida.realizar_jugada('c');
        assert_eq!(partida.letras_descubiertas, vec!['c']);
        assert_eq!(partida.intentos, 5);
    }
    #[test]
    fn el_juego_termina_cuando_se_descubren_todas_las_letras() {
        let mut partida = Partida::new("caso".to_string(), 5);
        partida.realizar_jugada('c');
        partida.realizar_jugada('a');
        partida.realizar_jugada('s');
        partida.realizar_jugada('o');
        assert!(!partida.continua_juego());
    }

    #[test]
    fn el_juego_no_termina_si_quedan_letras() {
        let mut partida = Partida::new("casa".to_string(), 5);
        partida.realizar_jugada('c');
        partida.realizar_jugada('a');
        assert!(partida.continua_juego());
    }

    #[test]
    fn solo_es_necesario_poner_la_letra_una_vez() {
        let mut partida = Partida::new("casa".to_string(), 5);
        partida.realizar_jugada('c');
        partida.realizar_jugada('a');
        partida.realizar_jugada('s');
        assert!(!partida.continua_juego());
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

// TODO:
// 1- Agregar prints de juego, "ingrese una letra", cantidad de intentos,
//  letras utilizadas, palabra descubierta al finalizar, mensaje de error al no ingresar una letra
// 2- Verificar casos borde, palabra nula, escribe muchas letras, etc.
// 3- Conectar la lectura del archivo de palabras con el juego (testear con test de integración??)
// 4- Eliminar warnings, correr clippy y fmt. Ver si es necesario que sea string el iniciar partida.
// 5. Podria desacoplarse también el output (sacar todos los print o println)
