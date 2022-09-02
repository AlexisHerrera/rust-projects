# Buscador full text

## Problemas encontrados

1. Investigué como testear la parte de _preparacion de los datos_ . Esto involucra mockear fs::File. Lo dejo para más adelante. Dejo algunas pistas de como hacerlo: 
[aquí](https://users.rust-lang.org/t/mocking-std-fs-for-unit-tests/22382) y 
[aquí](https://stackoverflow.com/questions/67554892/how-to-mock-stdfsfile-so-can-check-if-fileset-len-was-used-correctly-i).
Lamentablemente no fueron tan sencillas como para tomarme el tiempo de hacerlo.

2. Hago que Buscador tenga un metodo cargar_documento que recibe un repositorio_documentos para que en un futuro, si quiero cambiar la fuente de donde obtener los documentos solo deba cambiar la firma de RepositorioDocumentos para que reciba otro origen. En este caso está hardocodeado el Repositorio para que lo obtenga de una carpeta llamada documentos.

3. Por temas de tiempo, no voy a implementar una forma linda de obtener los terminos del documento en función de mantenerlo genérico.
Una forma linda sería buffereando los terminos con un vector, hacer getlines on demand, etc.

4. Utilice como base para corroborar un parcial de tf-idf que hice en orga de datos. Ver [aqui](parcial_tf-idf.pdf)