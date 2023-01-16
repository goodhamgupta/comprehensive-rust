// implement trait for a collection of mixed types using `trait objects`
// box => heap allocation
// dyn is used to highlight that calls to methods on the associated `Trait` are `dynamically
// dispatched`
// more expensive than impl Trait

use std::fmt::Display;

fn main() {
    let xs: Vec<Box<dyn Display>> = vec![Box::new(123), Box::new("Hello")];
    for x in xs {
        println!("x: {x}");
    }
}
