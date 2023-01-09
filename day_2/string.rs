fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity={}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity={}", s2.len(), s2.capacity());
}
