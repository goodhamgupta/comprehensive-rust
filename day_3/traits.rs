// Traits are similar to interfaces

trait Greet {
    fn say_hello(&self);
}

struct Dog {
    name: String,
}

impl Greet for Dog {
    fn say_hello(&self) {
        println!("bow");
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Leo"),
    };
    println!("Hello, my name is {} and i say", dog.name);
    dog.say_hello();
}
