fn main() {
    let s1: String = String::from("Hello!");
    // assignment transfers ownership
    // only when s2 goes out of scope, the string data is freed. since s1 does not have ownership,
    // when it goes out of scope, nothing happens!
    let s2: String = s1;
    println!("s2: {s2}");
}
