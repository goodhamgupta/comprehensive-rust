// while is similar to all other languages

fn main() {
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 }
    }
    println!("Final x: {x}");
}
