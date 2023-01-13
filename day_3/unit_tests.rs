// unit tests are supported throughout codebose
// integration tests are only supported in the `tests` folder

fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx], // get everything from 0 to idx-1
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello world"), "Hello")
}
