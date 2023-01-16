fn main() {
    let emojis = "🗻∈🌏";
    unsafe {
        // undefined behaviour if indices do not lie on utf-8 sequence boundaries.
        println!("{}", emojis.get_unchecked(0..4));
        println!("{}", emojis.get_unchecked(4..7));
        println!("{}", emojis.get_unchecked(7..11));
    }
}
