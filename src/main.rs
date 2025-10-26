mod dynamic;
use crate::dynamic::Pet;

fn dyn_call(p: &dyn Pet) {
    println!("{}", p.talk());
}

fn static_call(p: &impl Pet) {
    println!("{}", p.talk());
}

fn main() {
    let cat = dynamic::Cat::new(10);
    let dog = dynamic::Dog::new(1);
    dyn_call(&cat);
    static_call(&cat);
    assert_eq!(cat.talk(), "meowwwww".to_string());
    assert_eq!(cat.lives(), 10);
    dyn_call(&dog);
    static_call(&dog);
    assert_eq!(dog.talk(), "woof".to_string());
    assert_eq!(dog.lives(), 1);
}