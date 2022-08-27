struct TextoAPalabras;

impl TextoAPalabras {
    fn new() -> Self {
        TextoAPalabras {}
    }

    fn texto_a_palabras(&self, texto: String) -> Vec<String>{
        let mut texto_trimeado = texto.trim().to_string();
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
}