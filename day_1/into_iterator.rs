// IntotIterator tells us how to create the iterator
pub trait IntoIterator {
    type Item; // the type we iterate over
    type IntoIter: Iterator<Item = Self::Item>; // type returned by the into_iter method

    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");
}
