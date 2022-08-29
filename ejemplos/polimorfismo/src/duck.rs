pub struct Duck {
    name: String,
}

impl Duck {
    pub fn new(name : String) -> Duck {
        Duck { name }
    }
}

use crate::Animal;
impl Animal for Duck {
    fn sound(&self) -> String {
        format!("Cuack")
    }
}