pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name : String) -> Dog {
        Dog { name }
    }
}

use crate::Animal;
impl Animal for Dog {
    fn sound(&self) -> String {
        format!("Guau")
    }
}