pub fn calc(x: i32) -> i32 {
    match x {
        0 => 1,
        1 => 1,
        y => if y < 0 { 
                panic!("fib cannot take negative values.")
            } else {
                calc(y-1) + calc(y-2)
            }
    }
}