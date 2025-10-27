mod smartpointers;
use smartpointers::Pet;
use smartpointers::Cat;
use smartpointers::Dog;
fn main() {
    let mut pets: Vec<Box<dyn Pet>> = Vec::new();
    pets.push(Box::new(Dog::new("fido".to_string(), 10)));
    pets.push(Box::new(Cat::new(10)));
    for pet in pets {
        println!("{}", pet.talk());
    }
}
