fn main() {
    let t: (i32, i32, bool) = (2, 3, false);
    let (mut x,..,mut y) = t;
    println!("{}", x);
    println!("{}", y);
    (_, x, y) = t;
    println!("{}", x);
    println!("{}", y);
    let (.., w) = t;
    println!("{}", w); 
}