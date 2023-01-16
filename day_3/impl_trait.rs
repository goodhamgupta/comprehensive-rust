// similar to trait bounds, `impl Trait` syntax can be used in function arguments and return
// values
// impl trait cannot be used with ::<> syntax

use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let x = get_x("foo");
    println!("{x}");
}
