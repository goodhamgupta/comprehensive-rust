// generics lets us abstract an algorithm over the types used in the algorithm
// eg: sort method will do quick sort for array of integers, and it will do alphabetical sort for
// array of characters/strings

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.0, y: 10.0 };
    println!("{integer:#?} {float:#?}");
}
