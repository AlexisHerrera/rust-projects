use std::collections::HashMap;

use repositorio_documentos::RepositorioDocumentos;

struct Buscador {
    corpus: HashMap<String, HashMap<String, i32>>
}

impl Buscador {
    fn new() -> Buscador {
        Buscador {
            corpus: HashMap::new()
        }
    }

    fn cargar_documentos(&self, repositorio_documentos: RepositorioDocumentos) {
        // loop {
        //     // Crea un Documento y carga todos sus terminos en un vector
        //     let documento = repositorio_documentos.obtener_documento();
        //     if documento.is_none() { break }

        //     let documento = documento.unwrap();
        //     let nombre_documento = documento.nombre();

        //     // Consume los terminos del documento
        //     // Algun dia hacer refactor para no cargar todo el archivo
        //     for termino in documento.obtener_terminos() {
        //         self.agregar_corpus(termino, nombre_documento);
        //     }
        // }
    }

    fn agregar_corpus(&self, termino: String, nombre_documento : String) {

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
        let buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        
        let mut corpus_esperado: HashMap<String, i32> = HashMap::new();
        corpus_esperado.insert("doc1.txt".to_string(), 1);

        assert_eq!(buscador.corpus.len(), 1);
        assert_eq!(buscador.corpus.get(&"casa".to_string()), Some(&corpus_esperado));
    }
}

// Documentos es parecido a un File que tiene los metodos nombre, obtener_termino
