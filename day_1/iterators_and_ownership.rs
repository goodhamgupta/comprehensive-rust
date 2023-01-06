// traints are like interfacs. they describe behavior for a type
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next()); // iterator returns Some(T)
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next()); // iterator returns None when the array is
                                                  // empty
                                                  // Option is basically a nullable. Rust doesn't have null, but if you want to talk about a value that may not exist, you use Option<WhateverTypeThatValueIs>. When it does exist, it will be a Some(value), else None.
}
