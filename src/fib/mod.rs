pub fn calc(x: i32) -> i32 {
    if x == 0 || x== 1 {
        return 1
    }
    println!("{}", x);
    return calc(x-1) + calc(x-2)
}