#[derive(Debug, Clone, Default)]
pub struct Safe {
    value: i32
}

type Dollars = i32;

impl Safe {
    pub fn new(v: i32) -> Safe {
        Safe{value: v}
    }
}

pub trait ValueStore {
    type Currency;
    fn withdraw(self: &mut Self, amount: Self::Currency) -> Result<Self::Currency, &str>;
    fn deposit(self: &mut Self, amount: Self::Currency);
    fn balance(self: &Self) -> Result<Self::Currency, ()>;
}

impl ValueStore for Safe {
    type Currency = Dollars;
    fn withdraw(self: &mut Self, amount: Dollars) -> Result<Self::Currency, &str> {
        if self.value >= amount {
            self.value -= amount;
            Ok(amount)
        } else {
            Err("Insufficient funds.")
        }
    }

    fn deposit(self: &mut Self, amount: Self::Currency) {
        self.value += amount
    }

    fn balance(self: &Self) -> Result<Self::Currency, ()> {
        Ok(self.value)
    }
}
