mod collatz;

fn main() {
    let t: (i32, bool, [i32; 8]) = (2, true, [1,2,3,4,5,6,7,8]);
    println!("{}", t.0);
    println!("{}", t.1);
    for item in t.2 {
        println!("{}", item);
    }
}
