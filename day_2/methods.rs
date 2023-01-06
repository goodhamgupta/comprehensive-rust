#[derive(Debug)]

// use impl to define methods for structs
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        // &self => borrows the object immutably
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let test = Person {
        name: String::from("messi"),
        age: 35,
    };
    test.say_hello()
}
