// match keyword ets us match a value against one or more patterns
// comparisons are done top to bottom

fn main() {
    let input = 'a';

    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'd' | 'w' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        _ => println!("Something else"), // _ matches any value
    }
}
