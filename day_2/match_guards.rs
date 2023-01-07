// similar to guard functions in elixir
// add condition to match. the condition should always be a boolean expression

fn main() {
    let pair = (2, -3);
    println!("Tell me about {pair:?}");

    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum is 0"),
        (x, y) if x % 2 == 0 => println!("x is even"),
        _ => println!("ignored"),
    }
}
