enum BookError {
    NotAvailable,
    NotFound,
}

#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    is_available: bool,
}

impl Book {
    fn new(title: &str, author: &str) -> Self {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            is_available: true,
        }
    }

    fn display(&self) {
        println!(
            "Title: {}, Author: {}, Available: {}",
            self.title, self.author, self.is_available
        );
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn borrow_book(&mut self, title: &str) -> Result<(), BookError> {
        for book in &mut self.books {
            if book.title == title {
                if book.is_available {
                    book.is_available = false;
                    return Ok(());
                } else {
                    return Err(BookError::NotAvailable);
                }
            }
        }
        Err(BookError::NotFound)
    }

    fn find_book_by_title(&mut self, title: &str) -> Result<Book, BookError> {
        for book in &self.books {
            if book.title == title {
                return Ok((*book).clone());
            }
        }
        Err(BookError::NotFound)
    }

    fn display_books(&self) {
        for book in &self.books {
            book.display();
        }
    }
}

pub fn run_library_book() {
    println!("------- library book -------");
    let mut library = Library::new();

    library.add_book(Book::new("Book 1", "author 1"));
    library.add_book(Book::new("book 2", "author 2"));
    library.add_book(Book::new("1984", "author 2"));

    println!("All Books:");
    library.display_books();

    match library.borrow_book("Book 1") {
        Ok(()) => println!("Book borrowed successfully!"),
        Err(BookError::NotAvailable) => println!("Book is not available."),
        Err(BookError::NotFound) => println!("Book not found."),
    }

    println!("\nAll Books:");
    library.display_books();

    match library.find_book_by_title("book 2") {
        Ok(book) => {
            println!("\nFound Book:");
            book.display();
        }
        Err(BookError::NotFound) => println!("Book not found."),
        Err(_) => println!("An error occurred."),
    }
    println!("\n");
    println!("\n");
}
