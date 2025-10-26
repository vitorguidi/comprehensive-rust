pub fn length(x: i32) -> i32 {
    1 + match x {
        1 => 0,
        x if x%2 == 0 => length(x/2),
        _ => length (3*x + 1),
    }
}