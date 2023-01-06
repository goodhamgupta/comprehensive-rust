fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("String: {:?}", word);
    }
    for word in v {
        println!("String: {:?}", word);
    }
}
