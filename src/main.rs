fn main() {
    let s: String = "abcde".to_string();
    let t: &str = &s;

    dbg!(s);
    dbg!(t);
}