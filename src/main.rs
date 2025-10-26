mod collatz;

fn main() {
    let mut a: [i32; 5] = [1,2,3,4,5];
    a[2] = 6;
    for item in a { 
        println!("{}", item);
    }
}
