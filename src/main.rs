mod collatz;

fn main() {
    println!("{}", collatz::length(1));
    println!("{}", collatz::length(5));
    println!("{}", collatz::length(2));
    println!("{}", collatz::length(10));
}
