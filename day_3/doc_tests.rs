// documentation tests similar to elixir
// code blocks with /// are seen as rust code
// note that the vim binding :RustTest doesn't run the doctests

/// ```
/// use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");

pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}

fn main() {}
