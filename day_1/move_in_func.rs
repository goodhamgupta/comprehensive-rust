fn say_hello(name: String) {
    println!("Hello {name}");
}
fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // When a value is passed to a function, the value is assigned to the function parameter.
    // say_hello(name);
}
