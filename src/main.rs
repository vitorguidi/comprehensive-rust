use std::cmp::PartialOrd;

pub fn min<T: PartialOrd>(lval: T, rval: T) -> Result<T, ()> {
    if lval < rval {
       Ok(lval)
    } else {
       Ok(rval)
    }
}

fn main() {
    assert_eq!(min(2,3), Ok(2));
    assert_eq!(min("a","b"), Ok("a"));
}