use console::Term;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    println!("Press any key to continue...");
    let c = term.read_char()?;
    match c {
        '0'..='9' => println!("Your digit is a number: {}", c),
        'w' | 'a' | 's' | 'd' => println!("Your digit is a move: {}", c),
        key if key != '#' => println!("Not a #: {}", c),
        _ => println!("Hashtag!!! {}", c)
    }
    Ok(())
}