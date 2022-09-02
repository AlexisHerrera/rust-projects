use std::collections::HashMap;

use repositorio_documentos::RepositorioDocumentos;
#[derive(Debug)]
pub struct Buscador {
    corpus: HashMap<String, HashMap<String, i32>>,
    nombres_documentos: Vec<String>
}

pub struct DocumentoPuntaje {
    nombre_documento: String,
    puntaje: f32
}

impl DocumentoPuntaje {
    fn new(nombre_documento: String, puntaje: f32) -> DocumentoPuntaje {
        DocumentoPuntaje {
            nombre_documento,
            puntaje
        }
    }
}

impl Buscador {
    pub fn new() -> Buscador {
        Buscador {
            corpus: HashMap::new(),
            nombres_documentos: Vec::new()
        }
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
        if !self.nombres_documentos.contains(&nombre_documento) {
            self.nombres_documentos.push(nombre_documento.clone());
        }
        let hash_documentos_frecuencias = self.corpus.entry(termino).or_insert_with(HashMap::new);
        hash_documentos_frecuencias
            .entry(nombre_documento)
            .and_modify(|frecuencia| *frecuencia += 1)
            .or_insert(1);
    }

    /// Se utiliza el idf(t, D) = log(|D| + 1 / |{d pertenece D: t pertenece d}|) (variante custom)
    fn obtener_idf(&self, termino: &str) -> f32 {
        let numerador = self.cantidad_documentos() + 1;
        let divisor = self.cantidad_documentos_que_tienen_el_termino(termino);
        if numerador == 0 || divisor == 0 {
            return 0.0;
        }
        ((numerador as f32 / divisor as f32)).log10()
    }

    fn cantidad_documentos(&self) -> usize {
        self.nombres_documentos.len()
    }

    fn cantidad_documentos_que_tienen_el_termino(&self, termino: &str) -> usize {
        match self.corpus.get(termino) {
            Some(hash_terminos) => hash_terminos.len(),
            None => 0,
        }
    }

    fn obtener_tf(&self, termino: &str, nombre_documento: &str) -> i32 {
        if let Some(hash_documentos_frecuencia) = self.corpus.get(termino) {
            if let Some(frecuencia) = hash_documentos_frecuencia.get(nombre_documento) {
                return *frecuencia;
            }
        }
        0
    }

    fn obtener_tf_idf(&self, termino: &str, nombre_documento: &str) -> f32 {
        self.obtener_tf(termino, nombre_documento) as f32 *self.obtener_idf(termino)
    }

    pub fn realizar_busqueda(&self, terminos: &Vec<String>) -> Vec<DocumentoPuntaje> {
        let mut resultado = vec![];
        
        for nombre_documento in &self.nombres_documentos {
            let mut puntaje_documento = 0.0;
            for termino in terminos {
                puntaje_documento += self.obtener_tf_idf(termino, nombre_documento);
            }
            if puntaje_documento == 0.0 {
                continue;
            }
            resultado.push(DocumentoPuntaje::new((*nombre_documento).clone(), puntaje_documento));
        }
        resultado
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
    #[test]
    fn si_no_se_cargan_documentos_es_0_la_cantidad_de_documentos() {
        let buscador = Buscador::new();
        assert_eq!(buscador.cantidad_documentos(), 0);
    }

    #[test]
    fn si_se_carga_1_documento_es_1_la_cantidad_de_documentos() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        
        assert_eq!(buscador.cantidad_documentos(), 1);
    }

    #[test]
    fn si_se_carga_1_documento_y_muchos_terminos_solo_hay_1_documento() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("cielo".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("mar".to_string(), "doc1.txt".to_string());
        
        assert_eq!(buscador.cantidad_documentos(), 1);
    }

    #[test]
    fn se_cargan_muchos_documentos_con_varios_terminos_cuentan_sin_repetir() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("cielo".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("mar".to_string(), "doc1.txt".to_string());
        
        buscador.agregar_corpus("paz".to_string(), "doc2.txt".to_string());
        buscador.agregar_corpus("amor".to_string(), "doc2.txt".to_string());
        buscador.agregar_corpus("mar".to_string(), "doc2.txt".to_string());


        assert_eq!(buscador.cantidad_documentos(), 2);
    }
}

mod test_calculo_idf_custom {
    use super::*;
    
    fn calcular_idf_custom(cantidad_documentos : f32, cantidad_de_documentos_en_que_t_aparece: f32) -> f32 {
        ((cantidad_documentos + 1.0 )/cantidad_de_documentos_en_que_t_aparece).log10()
    }
    
    #[test]
    fn si_el_corpus_no_tiene_un_termino_idf_vale_0() {
        let buscador = Buscador::new();
        let termino_busqueda = "un-termino-que-no-existe";
        assert_eq!(buscador.obtener_idf(termino_busqueda), 0.0);
    }

    #[test]
    fn si_el_corpus_no_tiene_un_solo_documento_con_1_solo_termino_devuelve_0() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());

        let termino_busqueda = "casa";
        assert_eq!(buscador.obtener_idf(termino_busqueda), calcular_idf_custom(1.0, 1.0));
    }


    #[test]
    fn si_el_corpus_no_tiene_2_documentos_con_distintos_terminos() {
        let mut buscador = Buscador::new();

        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("cielo".to_string(), "doc2.txt".to_string());

        let termino_busqueda = "casa";
        assert_eq!(buscador.obtener_idf(termino_busqueda), calcular_idf_custom(2.0, 1.0));
    }

}

mod test_calculo_tf_clasico {
    use super::*;
    #[test]
    fn si_no_existe_el_termino_en_el_corpus_el_tf_es_0() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("otrotermino".to_string(), "doc1.txt".to_string());
        
        let termino_busqueda = "casa";
        let nombre_documento = "doc1.txt";

        assert_eq!(buscador.obtener_tf(termino_busqueda, nombre_documento), 0);
    }

    #[test]
    fn si_existe_el_termino_en_el_corpus_pero_no_es_del_documento_buscado_el_tf_es_0() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        
        let termino_busqueda = "casa";
        let nombre_documento = "doc2.txt";

        assert_eq!(buscador.obtener_tf(termino_busqueda, nombre_documento), 0);
    }

    #[test]
    fn si_existe_el_termino_en_el_corpus_y_es_el_documento_buscado_el_tf_es_la_frecuencia() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        
        let termino_busqueda = "casa";
        let nombre_documento = "doc1.txt";

        assert_eq!(buscador.obtener_tf(termino_busqueda, nombre_documento), 1);
    }

    #[test]
    fn obtiene_la_frecuencia_si_se_agrego_muchas_veces() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        
        let termino_busqueda = "casa";
        let nombre_documento = "doc1.txt";

        assert_eq!(buscador.obtener_tf(termino_busqueda, nombre_documento), 3);
    }
}

mod calcular_tf_idf {
    use super::*;
    fn calcular_idf_custom(cantidad_documentos : f32, cantidad_de_documentos_en_que_t_aparece: f32) -> f32 {
        ((cantidad_documentos + 1.0 )/cantidad_de_documentos_en_que_t_aparece).log10()
    }
    #[test]
    fn calcula_tf_idf_a_0_si_no_existe_el_termino() {
        let buscador = Buscador::new();

        let termino_busqueda = "casa";
        let nombre_documento = "doc1.txt";

        assert_eq!(buscador.obtener_tf_idf(termino_busqueda, nombre_documento), 0.0);
    }

    #[test]
    fn calcula_tf_idf_de_un_termino_y_documentos_existentes() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        
        buscador.agregar_corpus("casa".to_string(), "doc2.txt".to_string());
        buscador.agregar_corpus("amor".to_string(), "doc2.txt".to_string());
        buscador.agregar_corpus("amor".to_string(), "doc2.txt".to_string());



        let termino_busqueda = "casa";
        let nombre_documento = "doc1.txt";

        assert_eq!(buscador.obtener_tf_idf(termino_busqueda, nombre_documento), 3.0*calcular_idf_custom(2.0, 2.0));
    }
}

mod realizar_busqueda {
    use super::*;
    
    #[test]
    fn busqueda_de_termino_que_no_esta_en_documentos_no_devuelve_resultados() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("parque".to_string(), "doc2.txt".to_string());

        let termino_busqueda = vec!["unicornio".to_string()];
        assert!(buscador.realizar_busqueda(&termino_busqueda).is_empty());
        
    }

    #[test]
    fn busqueda_de_termino_que_solo_esta_en_un_documento_devuelve_el_nombre_del_documento() {
        let mut buscador = Buscador::new();
        buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        buscador.agregar_corpus("parque".to_string(), "doc2.txt".to_string());

        let termino_busqueda = vec!["casa".to_string()];
        let resultado_busqueda = buscador.realizar_busqueda(&termino_busqueda);
        assert_eq!(resultado_busqueda.len(), 1);
        assert_eq!(resultado_busqueda[0].nombre_documento, "doc1.txt");
    }

    #[test]
    fn busqueda_de_termino_que_esta_en_un_documento_muchas_veces_y_en_otro_solo_1_vez() {
        let mut buscador = Buscador::new();
        // Tiene que ganar por el tf
        for _ in 0..10 {
            buscador.agregar_corpus("casa".to_string(), "doc1.txt".to_string());
        }
        
        buscador.agregar_corpus("casa".to_string(), "doc2.txt".to_string());

        let termino_busqueda = vec!["casa".to_string()];
        let resultado_busqueda = buscador.realizar_busqueda(&termino_busqueda);
        assert_eq!(resultado_busqueda.len(), 2);
        assert_eq!(resultado_busqueda[0].nombre_documento, "doc1.txt");
        assert_eq!(resultado_busqueda[1].nombre_documento, "doc2.txt");
    }

}


// Quedan hacer dos cosas,
// 1. calcular el tf de los terminos (es decir la cantidad de veces que aparecen en los documentos)
// 2. Realizar la consulta y calcular los tf idf y rankearlos
// 3. Pregunta: cuando se devuelve el resultado desde un objeto (en este caso el buscador)