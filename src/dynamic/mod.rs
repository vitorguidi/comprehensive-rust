pub struct Dog {
    lives: i32
}

impl Dog {
    pub fn new(amt: i32) -> Dog {
        Dog{lives: amt}
    }
}

pub struct Cat {
    lives: i32
}

impl Cat {
    pub fn new(amt: i32) -> Cat {
        Cat{lives: amt}
    }
}

pub trait Pet {
    fn talk(self: &Self) -> String;
    fn lives(self: &Self) -> i32;
}

impl Pet for Dog {
    fn talk(self: &Self) -> String {
        "woof".to_string()
    }
    fn lives(self: &Self) -> i32 {
        self.lives
    }
}

impl Pet for Cat {
    fn talk(self: &Self) -> String {
        "meowwwww".to_string()
    }    
    fn lives(self: &Self) -> i32 {
        self.lives
    }
}