fn find_odd(v: &[i32]) -> Option<Vec<&i32>> {
    let odd: Vec<&i32> = v.iter()
        .filter(|&&x: &&i32| -> bool {x%2 == 1})
        .collect();
    if odd.len() == 0 {
        None
    } else {
        Some(odd)
    }
}
fn main() {
    let odd: [i32; 3] = [1,3,5];
    assert!(find_odd(&odd) != None);
    let even: [i32; 2] = [2,4];
    assert!(find_odd(&even) == None);
}