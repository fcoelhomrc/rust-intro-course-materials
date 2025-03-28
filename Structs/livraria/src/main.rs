use std::io;

#[derive(Debug, Clone)]
struct Book {
    // required fields
    isbn: String,
    title: String,
    author: String,
    keywords: Vec<String>,
    // auxiliary fields
    is_available: bool,
}

impl Book {
    fn new(isbn: &str, title: &str, author: &str, keywords: Vec<&str>) -> Self {
        Self {
            isbn: isbn.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            keywords: keywords.iter().map(|s| s.to_string()).collect(),
            is_available: true,
        }
    }
}

// had to implement traits manually bc otherwise "is_available" would be
// used for comparison... which is not what we want
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
            && self.title == other.title
            && self.author == other.author
            && self.keywords == other.keywords
    }
}

impl Eq for Book {}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
}

impl Library {
    // auxiliary methods
    fn find_book(&self, book: &Book) -> usize {
        self.books
            .iter()
            .position(|x| x == book)
            .expect("Could not find book")
    }

    fn find_all_books(&self, book: &Book) -> Vec<usize> {
        let indices = {
            self.books
                .iter()
                .enumerate()
                .filter(|(_, x)| **x == *book)
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
        };
        indices
    }

    // required methods
    fn add_book(&mut self, book: &Book) {
        self.books.push(book.clone()); // clone just to make testing less annoying
    }

    fn remove_book(&mut self, book: &Book) {
        let i = self.find_book(book);
        self.books.remove(i);
    }

    fn give_book(&mut self, book: &Book) {
        let indices = self.find_all_books(book);
        let mut success = false;
        for i in indices {
            if self.books[i].is_available {
                self.books[i].is_available = false;
                success = true;
                break;
            } else {
                continue;
            }
        }
        if !success {
            panic!("No books available!");
        }
    }

    fn receive_book(&mut self, book: &Book) {
        let indices = self.find_all_books(book);
        let mut success = false;
        for i in indices {
            if !self.books[i].is_available {
                self.books[i].is_available = true;
                success = true;
                break;
            }
        }
        if !success {
            println!("Thanks for your donation! :)");
            self.add_book(book);
        }
    }
}

fn create_examples() -> (Book, Book, Book) {
    let book1 = Book::new(
        "978-3-16-148410-0",
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        vec!["rust", "programming", "systems"],
    );

    let book2 = Book::new(
        "978-0-201-83595-3",
        "The C Programming Language",
        "Brian W. Kernighan and Dennis M. Ritchie",
        vec!["c", "programming", "classic"],
    );

    let book3 = Book::new(
        "978-1-59327-599-0",
        "Automate the Boring Stuff with Python",
        "Al Sweigart",
        vec!["python", "automation", "beginner"],
    );
    (book1, book2, book3)
}

// cli stuff
fn option_menu() {
    println!("0: quit");
    println!("1: create new Book entry");
    println!("2: delete an existing Book entry");
    println!("3: request a Book");
    println!("4: return a Book");
    println!("5: consult Library");
}

fn option_input() -> u32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<u32>().unwrap()  // typecast to u32
}

fn string_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn book_input() -> Book {
    println!("Please enter your book ISBN code: ");
    let isbn = string_input();
    println!("Please enter your book title: ");
    let title = string_input();
    println!("Please enter your book author: ");
    let author = string_input();
    println!("Please enter its keywords (whitespace separated): ");
    let keywords = string_input();
    let keywords : Vec<&str> = keywords.split_whitespace().collect();

    Book::new(
        &isbn,
        &title,
        &author,
        keywords,
    )
}


fn main() {

    let (book1, book2, book3) = create_examples();
    let mut lib = Library {
        books: vec![book1, book2, book3],
    };

    loop {
        println!("Choose an option:");
        option_menu();
        let option = option_input();

        match option {
            1 => {
                let book = book_input();
                lib.add_book(&book)
            },
            2 => {
                let book = book_input();
                lib.remove_book(&book)
            },
            3 => {
                let book = book_input();
                lib.give_book(&book)
            },
            4 => {
                let book = book_input();
                lib.receive_book(&book)
            },
            5 => println!("{:#?}", lib),
            _ => break,
        }
    }

}





#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let (book1, book2, book3) = super::create_examples();

        println!("Book 1: {} by {}", book1.title, book1.author);
        println!("Book 2: {} by {}", book2.title, book2.author);
        println!("Book 3: {} by {}", book3.title, book3.author);

        let mut lib = super::Library {
            books: vec![book1.clone(), book2.clone()],
        };

        lib.add_book(&book3);
        println!("{:#?}", lib);

        lib.remove_book(&book3);
        println!("{:#?}", lib);

        lib.give_book(&book1);
        println!("{:#?}", lib);

        lib.receive_book(&book1);
        println!("{:#?}", lib);

        lib.add_book(&book1);
        lib.add_book(&book1);
        lib.give_book(&book1);
        lib.give_book(&book1);
        println!("{:#?}", lib);

        lib.receive_book(&book3);
        println!("{:#?}", lib);
    }
}
