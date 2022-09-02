use std::collections::HashMap;

use repositorio_documentos::RepositorioDocumentos;
#[derive(Debug)]
pub struct Buscador {
    corpus: HashMap<String, HashMap<String, i32>>,
}

impl Buscador {
    pub fn new() -> Buscador {
        Buscador {
            corpus: HashMap::new(),
        }
    }

    fn inyectar_corpus(&mut self, corpus: HashMap<String, HashMap<String, i32>>) {
        self.corpus = corpus;
    }

    pub fn cargar_documentos(&mut self, mut repositorio_documentos: RepositorioDocumentos) {
        loop {
            // Crea un Documento y carga todos sus terminos en un vector
            let documento = match repositorio_documentos.obtener_documento() {
                Err(_error) => break,
                Ok(documento) => documento,
            };

            let nombre_documento = documento.nombre;
            // Consume los terminos del documento
            // Algun dia hacer refactor para no cargar todo el archivo
            for termino in documento.terminos {
                self.agregar_corpus(termino, nombre_documento.clone());
            }
        }
    }

    fn agregar_corpus(&mut self, termino: String, nombre_documento: String) {
        let hash_documentos_frecuencias = self.corpus.entry(termino).or_insert_with(HashMap::new);
        hash_documentos_frecuencias
            .entry(nombre_documento)
            .and_modify(|frecuencia| *frecuencia += 1)
            .or_insert(1);
    }

    /// Se utiliza el idf(t, D) = log(|D|/ |{d pertenece D: t pertenece d}|) (variante clasica)
    fn obtener_idf(&self, termino: &str) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests_agregar_corpus {
    use super::*;
    #[test]
    fn corpus_esta_vacio_si_no_se_le_agrego_nada() {
        let buscador = Buscador::new();
        assert!(buscador.corpus.is_empty());
    }

    #[test]
    fn buscador_guarda_elemento_en_el_corpus() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());

        let corpus_esperado = HashMap::from([(
            "casa".to_string(),
            HashMap::from([("doc1.txt".to_string(), 1)]),
        )]);

        assert_eq!(buscador.corpus.len(), 1);
        assert_eq!(buscador.corpus, corpus_esperado);
    }

    #[test]
    fn buscador_guarda_repeticiones_en_el_corpus() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());

        let corpus_esperado = HashMap::from([(
            "casa".to_string(),
            HashMap::from([("doc1.txt".to_string(), 2)]),
        )]);

        assert_eq!(buscador.corpus.len(), 1);
        assert_eq!(buscador.corpus, corpus_esperado);
    }

    #[test]
    fn buscador_guarda_repeticiones_de_multiples_terminos() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("cielo".to_string(), "doc1.txt".to_string());

        buscador.agregar_corpus("casa".to_string(), "doc2.txt".to_string());
        buscador.agregar_corpus("mar".to_string(), "doc2.txt".to_string());

        let corpus_esperado = HashMap::from([
            (
                "casa".to_string(),
                HashMap::from([("doc1.txt".to_string(), 1), ("doc2.txt".to_string(), 1)]),
            ),
            (
                "cielo".to_string(),
                HashMap::from([("doc1.txt".to_string(), 1)]),
            ),
            (
                "mar".to_string(),
                HashMap::from([("doc2.txt".to_string(), 1)]),
            ),
        ]);

        assert_eq!(buscador.corpus, corpus_esperado);
    }
}

mod test_calculo_puntajes {
    use super::*;
    
    fn calcular_idf(cantidad_documentos : f32, cantidad_de_documentos_en_que_t_aparece: f32) -> f32 {
        (cantidad_documentos/cantidad_de_documentos_en_que_t_aparece).log10()
    }
    
    #[test]
    fn si_el_corpus_no_tiene_un_termino_idf_vale_0() {
        let buscador = Buscador::new();
        let termino_busqueda = "un-termino-que-no-existe";
        assert_eq!(buscador.obtener_idf(termino_busqueda), calcular_idf(0.0, 0.0));
    }

    #[test]
    fn si_el_corpus_no_tiene_un_solo_documento_con_1_solo_termino_devuelve_0() {
        let mut buscador = Buscador::new();
        let corpus  = HashMap::from([(
            "casa".to_string(),
            HashMap::from([("doc1.txt".to_string(), 1)]),
        )]);
        buscador.inyectar_corpus(corpus);
        let termino_busqueda = "casa";
        assert_eq!(buscador.obtener_idf(termino_busqueda), calcular_idf(1.0, 1.0));
    }


    #[test]
    fn si_el_corpus_no_tiene_2_documentos_con_distintos_terminos() {
        let mut buscador = Buscador::new();
        let corpus  = HashMap::from([(
            "casa".to_string(),
            HashMap::from([("doc1.txt".to_string(), 1)]),
            "cielo".to_string(),
            HashMap::from([("doc2.txt".to_string(), 1)]),
        )]);
        buscador.inyectar_corpus(corpus);
        let termino_busqueda = "casa";
        assert_eq!(buscador.obtener_idf(termino_busqueda), calcular_idf(2.0, 1.0));
    }

}
// Documentos es parecido a un File que tiene los metodos nombre, obtener_termino
