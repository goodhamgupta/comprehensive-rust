// panic: unrecoverable or unexpected error
// Use non-panicking API if possible
fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]); // causes panic
}
