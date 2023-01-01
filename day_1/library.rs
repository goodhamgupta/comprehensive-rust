#![allow(unused_variables, dead_code)]

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// This makes it possible to print Book balues with {}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library { books: vec![] }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        if self.len() == 0 {
            true
        } else {
            false
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn print_books(&self) {
        for book in self.books.iter() {
            println!("Book: {book}");
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|p| p.year)
    }
}

fn main() {
    let mut library: Library = Library::new();
    println!("Our library is empty: {}", &library.is_empty());
    let _ = &library.add_book(Book::new("Lord of the Rings", 1954));
    let _ = &library.add_book(Book::new("Alice adventure", 1865));

    &library.print_books();

    match &library.oldest_book() {
        Some(book) => println!("My oldest book is {book}"),
        None => println!("My library is empty!"),
    }

    println!("Our library has {} books", &library.len());
}
