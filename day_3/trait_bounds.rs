// help limit the types that can interact with a trat
//
// duplicate will only work for structs that have implemented the trait Clone
// UnClonable structs will not be able to use duplicate

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

struct NotClonable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    let nc = NotClonable {};
    // duplicate(nc); // this line will cause an error
    println!("{:#?}", pair);
}
