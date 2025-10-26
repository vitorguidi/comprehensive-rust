fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
    let result = match (x,y) {
        (x,y) if y!=0 => Ok(x/y),
        _ => Err("Cannot divide by zero.")
    };
    match result {
        Ok(value) => println!("Division successful: {}/{} = {}", x, y, value),
        Err(msg) => println!("Error: {}", msg)
    };
    result
}

fn main() {
    assert_eq!(Err("Cannot divide by zero."), divide(2,0));
    assert_eq!(divide(2,2),Ok(1));
}