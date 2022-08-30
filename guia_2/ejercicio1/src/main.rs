use std::mem;

fn main() {
    caso1();
    //drip_drop();
    caso3()
}

fn caso1() {
    let mut s = String::from("hola");
    mem::swap(&mut s, &mut String::from("chau"));
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    println!("{}", ref3.to_uppercase());
}

/// No se puede devolver una referencia de ese string porque
/// cuando termina la función, s se dropea y la referencia ya
/// no es válida. 
// fn drip_drop() -> &String {
//     let s = String::from("hello world!");
//     return &s;
// }

/// No se puede hacer s2 = v[0] ya que debería copiar v[0]
/// a s2 pero String no implementa ese trait.
fn caso3() {
    let s1 = String::from("hola");
    let mut v = Vec::new();
    v.push(s1);
    // let s2: String = v[0];
    let s2: String = v.pop().unwrap();
    println!("{}", s2);
}