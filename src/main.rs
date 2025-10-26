fn main() {
    let a: [i32; 3] = [1,2,3];
    let r = &a[0..2];
    dbg!(r);
    for item in r {
        println!("{}", item);
    }
    let mut b: [i32; 3] = [4,5,6];
    let s = &mut b[0..2];
    s[0]=10;
    s[1]=20;
    dbg!(s);
    dbg!(b);
}