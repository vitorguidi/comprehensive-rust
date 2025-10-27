#[derive(Debug)]
pub enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

impl<T: Clone> List<T> {
    pub fn append(self: Self, val: T) -> List<T> {
        List::Element(val, Box::new(self))
    }
    pub fn new() -> List<T> {
        List::Nil
    }
}

pub struct Dog {
    name: String,
    age: i32
}

impl Dog {
    pub fn new(name: String, age: i32) -> Dog {
        Dog{name: name, age: age}
    }
}

pub struct Cat {
    lives: i32
}

impl Cat {
    pub fn new(lives: i32) -> Cat {
        Cat{lives: lives}
    }
}

pub trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, mein name ist {}", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        "Meow".to_string()
    }
}