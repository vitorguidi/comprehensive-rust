fn main() {
    let mut x: i32 = 100;
    while x > 0 {
        println!("{}", x);
        x/= 2;
    }
    for x in 1..10 {
        println!("{}", x)
    }
    for val in [1,2,3,4,5] {
        println!("{}", val)
    }
}
