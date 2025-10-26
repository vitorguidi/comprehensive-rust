use std::cell::Cell;

fn main() {
    let val: Cell<i32> = Cell::new(0);
    let increment = |x: i32| {
        val.set(val.get() + x)
    };
    increment(1);
    assert_eq!(val.get(), 1);
    increment(5);
    assert_eq!(val.get(), 6);
    increment(-6);
    assert_eq!(val.get(), 0);
}