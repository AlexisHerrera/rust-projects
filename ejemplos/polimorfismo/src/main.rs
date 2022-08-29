mod duck;
mod dog;

pub trait Animal {
    fn sound(&self) -> String;
}   
use duck::Duck;
use dog::Dog;
fn main() {
    //static_binding();
    //dynamic_binding1();
    dynamic_binding2();
}

fn static_binding() {
    let duck = Duck::new("Patito".to_string());
    let dog = Dog::new("Perrito".to_string());
    println!("{}",duck.sound());
    println!("{}",dog.sound());
}

fn dynamic_binding1() {
    let duck = Duck::new("Patito".to_string());
    let dog = Dog::new("Perrito".to_string());
    let animals: Vec<&dyn Animal> = vec![&duck, &dog];
    for animal in animals {
        println!("{}",animal.sound());
    }
}
use rand::thread_rng;
use rand::seq::SliceRandom;
fn dynamic_binding2() {
    let duck = Duck::new("Patito".to_string());
    let dog = Dog::new("Perrito".to_string());
    let animals: Vec<&dyn Animal> = vec![&duck, &dog];

    let mut rng = thread_rng();
    let chosen = animals.choose(&mut rng).unwrap();
    println!("{}", chosen.sound());
}
