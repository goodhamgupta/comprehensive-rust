fn main() {
    // Some types are copied by default, instead of moving their ownership.
    // These types implement the `Copy` trait.
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");
}
