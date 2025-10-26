mod counter;
use counter::Counter;

fn main() {
    let mut ctr = counter::HashMapCounter::new();
    ctr.increment("a");
    assert_eq!(ctr.count("a"), 1);
    assert_eq!(ctr.count("b"), 0);
    ctr.increment("a");
    ctr.increment("b");
    assert_eq!(ctr.count("a"), 2);
    assert_eq!(ctr.count("b"), 1);
}