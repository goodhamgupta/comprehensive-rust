// match a value against a pattern using `if let`
fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("missing name?");
    }
}
