use std::io;

pub struct ErrorJugada;

pub struct Partida {
    intentos: i32
}

impl Partida {
    // Resta un intento
    fn reducir_intentos(&mut self) {
        self.intentos = self.intentos - 1;
    }

    // Devuelve true si quedan intentos, false en caso contrario
    fn quedan_intentos(&self) -> bool {
        self.intentos > 0
    }

    // Hay que testear esto con dependency injection
    // https://jeffkreeftmeijer.com/rust-stdin-stdout-testing/
    // ver si se puede parametrizar
    // ver mas casos de error (largo mayor a 0, numeros etc)
    pub fn obtener_jugada() -> Result<char, ErrorJugada > {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(bytes) => {
                println!("Se leyeron {} bytes", bytes);
                Ok('c')
            },
            Err(_) => {
                println!("Error al leer input");
                Err(ErrorJugada)
            }
        }
        

    }
}


#[cfg(test)]
mod tests {
    use super::Partida;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn disminuye_intentos() {
        let mut partida = Partida {
            intentos: 5
        };
        partida.reducir_intentos();
        assert_eq!(partida.intentos, 4);
    }

    #[test]
    fn quedan_intentos() {
        let mut partida = Partida {
            intentos: 1
        };
        partida.reducir_intentos();
        assert!(!partida.quedan_intentos());
    }


}