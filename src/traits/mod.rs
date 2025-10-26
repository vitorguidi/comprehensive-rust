pub struct Safe {
    value: i32
}

impl Safe {
    pub fn new(v: i32) -> Safe {
        Safe{value: v}
    }

    pub fn withdraw(self: &mut Safe, amount: i32) -> Result<i32, &str> {
        if self.value >= amount {
            self.value -= amount;
            Ok(amount)
        } else {
            Err("Insufficient funds.")
        }
    }

    pub fn deposit(self: &mut Safe, amount: i32) {
        self.value += amount
    }
}
