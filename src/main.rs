mod smartpointers;
use smartpointers::List;

fn main() {
    let mut l = smartpointers::List::new();
    l = l.append(2);
    l = l.append(3);
    dbg!(l);
}