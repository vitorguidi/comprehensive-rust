mod traits;
use crate::traits::Safe;
use crate::traits::ValueStore;

fn main() {
    let mut s: Safe = Safe::new(0);
    s.deposit(10);
    assert_eq!(s.withdraw(5), Ok(5));
    assert_eq!(s.withdraw(6), Err("Insufficient funds."));
}