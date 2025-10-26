pub struct Safe {
    value: i32
}

impl Safe {
    pub fn new(v: i32) -> Safe {
        Safe{value: v}
    }
}

pub trait ValueStore {
    fn withdraw(self: &mut Self, amount: i32) -> Result<i32, &str>;
    fn deposit(self: &mut Self, amount: i32);
}

impl ValueStore for Safe {

    fn withdraw(self: &mut Self, amount: i32) -> Result<i32, &str> {
        if self.value >= amount {
            self.value -= amount;
            Ok(amount)
        } else {
            Err("Insufficient funds.")
        }
    }

    fn deposit(self: &mut Self, amount: i32) {
        self.value += amount
    }
}
