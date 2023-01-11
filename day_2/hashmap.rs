use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of huckleberry finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);
    page_counts.insert("shady book".to_string(), 10);

    if !page_counts.contains_key("Les Miserables") {
        println!(
            "We've known about {} books, but not Les Miserables.",
            page_counts.len()
        );
    }

    for book in [
        "Pride and Prejudice",
        "Alice's Adventure in Wonderland",
        "shady book",
    ] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("Book is unknown"),
        }
    }
}
