mod collatz;

fn main() {
    let t: (i32, bool) = (2, true);
    let (val, boolval) = t;
    println!("{}", val);
    println!("{}", boolval);
    let [x,y,z] = [1,2,3];
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
