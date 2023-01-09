fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("cat stuff"),
        Some("ls") => println!("ls stuff"),
        Some("mv") => println!("mv stuff"),
        Some("rm") => println!("rm stuff"),
        None => println!("no"),
        _ => println!("unknown"),
    }
}
