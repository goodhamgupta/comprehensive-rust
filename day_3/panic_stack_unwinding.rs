use std::panic;

// catch panic stack trace
// useful when servers should keep running even if a single request crashes
// similar to process failure handing in erlang/elixir
fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello");
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });

    assert!(result.is_err());
}
