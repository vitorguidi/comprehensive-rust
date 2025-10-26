fn process(t: (i32, i32)) -> Result<i32, String> {
    match t {
        (x,y) if y != 0 => Ok(x/y),
                      _ => Err("Division by zero not allowed.".to_string())
    }
}

fn main() {
    let inputs: [(i32,i32); 4] = [(1,1), (2,2), (3,3), (4,0)];
    for input in inputs {
        if let Ok(result) = process(input) {
            println!("{}", result);
        } else {
            println!("Div by zero.");
            break;
        }
    }
    let all_results: Vec<Result<i32, String>> = inputs.iter()
        .map(|&input| process(input))
        .collect();
    dbg!(all_results);
}