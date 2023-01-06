struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("test"),
        age: 10,
    };
    println!("{} is {} years old", person.name, person.age);
}
