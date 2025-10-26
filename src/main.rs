fn process(t: (i32, i32)) -> Result<i32, String> {
    match t {
        (x,y) if y != 0 => Ok(x/y),
                      _ => Err("Division by zero not allowed.".to_string())
    }
}

fn main() {
    let inputs: [(i32,i32); 5] = [(0,0), (1,1), (2,2), (3,3), (4,4)];
    let mut all_results: Vec<Result<i32, String>> = inputs.iter()
        .map(|&input| process(input))
        .collect();
    while let Some(Ok(result)) = all_results.pop() {
        println!("{}", result);
    }
}