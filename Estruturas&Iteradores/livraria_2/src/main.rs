use std::collections::{HashMap, HashSet};
///
/// Livraria 2.0
///
/// Para implementar esta iteração do exercício, deve copiar a versão anterior livraria 1.0
/// E fazer todas as alterações pedidas pelo enunciado.
///
/// Devem manter ambas as versões do exercício.

/// Augmente a livraria feita anteriormente com estruturas de dados para eficientemente encontrar um livro pelo seu título ou ISBN e encontrar os livros escritos por um autor. Introduzir procura por palavras chave eficiente com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.

// FIXME:  WHEN I REMOVE BOOK I MESS UP THE INDICES!!!!

// TODO: add data structures
// TODO: find book by title
// TODO: find book by ISBN
// TODO: find all books from author
// TODO: find books by keyword
// TODO: ---- keyword intersection (kw1 AND kw2)
// TODO: ---- keyword union (kw1 OR kw2)
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

    // HashMap -> + efficient to check key membership && key retrieval
    //            - one hashmap per field (mem usage is approx. N times larger, N = num fields)
    // HashSet -> + no dupes (book idx in Library.books is unique)
    //            + efficient to check membership
    //            + efficient to add/remove value
    //            - inefficient to iterate
    books_by_title: HashMap<String, HashSet<usize>>,
    books_by_isbn: HashMap<String, HashSet<usize>>,
    books_by_author: HashMap<String, HashSet<usize>>,
    books_by_keyword: HashMap<String, HashSet<usize>>,
}

impl Library {
    // V2.0 - constructor
    fn new(books: Vec<Book>) -> Self {
        Self {
            books,
            books_by_title: HashMap::new(),
            books_by_isbn: HashMap::new(),
            books_by_author: HashMap::new(),
            books_by_keyword: HashMap::new(),
        }
    }

    // FIXME:  not very elegant having to call this manually after instantiation...
    //         but trying to use this inside Library::new is impossible due to borrow checker
    //         (double mut borrow...)
    //         Perhaps alternative is to copy paste the implementation of Library::update_hashmaps_on_add
    //         into the Library::new, in an attempt to have one mut borrow end before the other...
    //         but since this leads to code duplication, also not very elegant!
    fn update_hashmaps_on_new(&mut self) {
        let books_copy = self.books.clone();
        for (idx, book) in books_copy.iter().enumerate() {
            self.update_hashmaps_on_add(book, idx);
        }
    }

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

    // V2.0
    // FIXME: finish this implementation
    //        I think Library.books should stop being a vec
    //        and instead it should be a HashMap too (usize, Book)
    //        This way we can find_book_by_title -> index
    //        then retrieve book efficiently Library.books[index]
    fn find_book_by_title(&self, title: &str) -> Book {
        let idx = self
            .books_by_title
            .get(title)
            .expect("Could not find book")
            .iter()
            .next()
            .copied();

        if let Some(idx) = idx {
            self.books[idx].clone()
        } else {
            panic![
                "Found book with matching title, but its index was not registered in the HashMap"
            ]
        }
    }

    // V2.0
    fn find_book_by_isbn(&self, isbn: &str) -> Book {
        let idx = self
            .books_by_isbn
            .get(isbn)
            .expect("Could not find book")
            .iter()
            .next()
            .copied();

        if let Some(idx) = idx {
            self.books[idx].clone()
        } else {
            panic!["Found book with matching ISBN, but its index was not registered in the HashMap"]
        }
    }

    // V2.0
    fn find_all_books_by_author(&self, author: &str) -> Vec<Book> {
        let indices = self
            .books_by_author
            .get(author)
            .expect("Could not find book");

        let mut books = Vec::<Book>::new();
        for idx in indices {
            books.push(self.books[*idx].clone());
        }
        books
    }

    // V2.0
    fn find_books_by_keyword(&self, query: &str) -> Vec<Book> {
        // query should be on the following format
        // "keyword1 AND keyword2"
        // "keyword1 OR keyword2"

        let special_tokens = vec!["AND", "OR"];
        let parsed_string = query.split_whitespace().collect::<Vec<&str>>();

        let keywords = parsed_string
            .iter()
            .filter(|keyword| !special_tokens.contains(keyword))
            .map(|keyword| *keyword)
            .collect::<Vec<&str>>();

        let operators = parsed_string
            .iter()
            .filter(|operator| special_tokens.contains(operator))
            .map(|keyword| *keyword)
            .collect::<Vec<&str>>();

        // HashSet here will make it simpler to implement
        // the AND / OR operators (intersection / union)
        let mut keyword_indices = Vec::<&HashSet<usize>>::new();
        for keyword in keywords.iter() {
            let indices = self
                .books_by_keyword
                .get(*keyword)
                .expect("Could not find book");
            keyword_indices.push(indices);
        }

        let mut processed_indices = HashSet::<usize>::new();
        for (i, (kw, kw_idx)) in keywords
            .into_iter()
            .zip(keyword_indices.into_iter())
            .enumerate() {

            if i > 0 {
                match operators[i - 1] {
                    "AND" => {
                        processed_indices = processed_indices
                            .intersection(kw_idx)
                            .copied()
                            .collect::<HashSet<_>>();
                    },
                    "OR" => {
                        processed_indices = processed_indices
                            .union(kw_idx)
                            .copied()
                            .collect::<HashSet<_>>();
                    },
                    _ => panic!["Found non-valid operator in query!"],
                }
            } else {
                processed_indices.extend(kw_idx);
            }
        }


        let mut books = Vec::<Book>::new();
        for idx in processed_indices {
            books.push(self.books[idx].clone());
        }
        books
    }

    // required methods
    fn add_book(&mut self, book: &Book) {
        self.books.push(book.clone()); // clone just to make testing less annoying

        let idx = self.books.len() - 1;
        self.update_hashmaps_on_add(book, idx);
    }

    fn update_hashmaps_on_add(&mut self, book: &Book, idx: usize) {
        // update HashMaps
        if self.books_by_title.contains_key(&book.title) {
            self.books_by_title
                .get_mut(&book.title)
                .unwrap()
                .insert(idx);
        } else {
            self.books_by_title
                .insert(book.title.clone(), HashSet::new());
            self.books_by_title
                .get_mut(&book.title)
                .unwrap()
                .insert(idx);
        }

        if self.books_by_isbn.contains_key(&book.isbn) {
            self.books_by_isbn.get_mut(&book.isbn).unwrap().insert(idx);
        } else {
            self.books_by_isbn.insert(book.isbn.clone(), HashSet::new());
            self.books_by_isbn.get_mut(&book.isbn).unwrap().insert(idx);
        }

        if self.books_by_author.contains_key(&book.author) {
            self.books_by_author
                .get_mut(&book.author)
                .unwrap()
                .insert(idx);
        } else {
            self.books_by_author
                .insert(book.author.clone(), HashSet::new());
            self.books_by_author
                .get_mut(&book.author)
                .unwrap()
                .insert(idx);
        }

        for keyword in book.keywords.iter() {
            if self.books_by_keyword.contains_key(keyword) {
                self.books_by_keyword.get_mut(keyword).unwrap().insert(idx);
            } else {
                self.books_by_keyword
                    .insert(keyword.to_string().clone(), HashSet::new());
                self.books_by_keyword.get_mut(keyword).unwrap().insert(idx);
            }
        }
    }

    fn remove_book(&mut self, book: &Book) {
        let idx = self.find_book(book);
        self.books.remove(idx);

        self.update_hashmaps_on_remove(book, idx);
        self.cleanup_on_remove();
    }

    // DONE:  the HashSets might be left empty!
    //        Need to implement cleanup method that deletes keys from HashMaps
    //        when their HashSets are empty
    //        + memory cleanup
    //        - inefficient to iterate over HashMap
    fn update_hashmaps_on_remove(&mut self, book: &Book, idx: usize) {
        // update HashMaps
        if self.books_by_title.contains_key(&book.title) {
            self.books_by_title
                .get_mut(&book.title)
                .unwrap()
                .remove(&idx);
        }

        if self.books_by_isbn.contains_key(&book.isbn) {
            self.books_by_isbn.get_mut(&book.isbn).unwrap().remove(&idx);
        }

        if self.books_by_author.contains_key(&book.author) {
            self.books_by_author
                .get_mut(&book.author)
                .unwrap()
                .remove(&idx);
        }

        for keyword in book.keywords.iter() {
            if self.books_by_keyword.contains_key(keyword) {
                self.books_by_keyword.get_mut(keyword).unwrap().remove(&idx);
            }
        }
    }

    fn cleanup_on_remove(&mut self) {
        // temporary list of mutable borrows of each HashMap
        let mut hashmaps = vec![
            &mut self.books_by_title,
            &mut self.books_by_isbn,
            &mut self.books_by_author,
            &mut self.books_by_keyword,
        ];

        for hashmap in hashmaps.into_iter() {
            // consumes the list

            // IMMUTABLE STEP
            let to_clean = hashmap // finds empty HashSets
                .iter()
                .filter(|(k, v)| v.is_empty())
                .map(|(k, _)| k.clone())
                .collect::<Vec<_>>();

            //  MUTABLE STEP
            for k in to_clean.iter() {
                // for each empty HashSet
                hashmap.remove(k); // remove key from HashMap in place
            }
        }
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
    }
}

fn create_examples() -> (Book, Book, Book, Book) {
    let book1 = Book::new(
        "978-3-16-148410-0",
        "The Rust Programming Language",
        "Steve Klabnik",
        vec!["rust", "programming", "systems"],
    );

    let book2 = Book::new(
        "978-0-201-83595-3",
        "The C Programming Language",
        "Brian W. Kernighan",
        vec!["c", "programming", "classic"],
    );

    let book3 = Book::new(
        "978-1-59327-599-0",
        "Automate the Boring Stuff with Python",
        "Al Sweigart",
        vec!["python", "automation", "beginner"],
    );

    let book4 = Book::new(
        "978-0134190440",
        "The Go Programming Language",
        "Brian W. Kernighan",
        vec!["go", "programming", "concurrency", "goroutines", "channels"],
    );

    (book1, book2, book3, book4)
}

// cli stuff
fn option_menu() {
    println!("0: quit");
    println!("1: create new Book entry");
    println!("2: delete an existing Book entry");
    println!("3: request a Book");
    println!("4: return a Book");
    println!("5: search Book by title");
    println!("6: search Book by ISBN");
    println!("7: search Book by author");
    println!("8: search Book by keyword");
    println!("9: consult Library");
}

fn option_input() -> u32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    input.parse::<u32>().unwrap() // typecast to u32
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
    let keywords: Vec<&str> = keywords.split_whitespace().collect();

    Book::new(&isbn, &title, &author, keywords)
}

fn main() {
    let (book1, book2, book3, book4) = create_examples();
    let mut lib = Library::new(vec![book1, book2, book3, book4]);
    lib.update_hashmaps_on_new();

    loop {
        println!("Choose an option:");
        option_menu();
        let option = option_input();

        match option {
            1 => {
                let book = book_input();
                lib.add_book(&book)
            }
            2 => {
                let book = book_input();
                lib.remove_book(&book)
            }
            3 => {
                let book = book_input();
                lib.give_book(&book)
            }
            4 => {
                let book = book_input();
                lib.receive_book(&book)
            }
            5 => {
                println!("Input title:");
                let key = string_input();
                let book = lib.find_book_by_title(key.as_str());
                println!("Search result: {:#?}", book);
            }
            6 => {
                println!("Input ISBN:");
                let key = string_input();
                let book = lib.find_book_by_isbn(key.as_str());
                println!("Search result: {:#?}", book);
            }
            7 => {
                println!("Input author:");
                let key = string_input();
                let books = lib.find_all_books_by_author(key.as_str());
                println!("Search result: ");
                for book in books {
                    println!("{:#?}", book);
                }
            }
            8 => {
                println!("Input keywords (whitespace separated, supports OR and AND operators):");
                let key = string_input();
                let books = lib.find_books_by_keyword(key.as_str());
                println!("Search result: ");
                for book in books {
                    println!("{:#?}", book);
                }
            }
            9 => println!("{:#?}", lib),
            _ => break,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let (book1, book2, book3, book4) = super::create_examples();

        println!("Book 1: {} by {}", book1.title, book1.author);
        println!("Book 2: {} by {}", book2.title, book2.author);
        println!("Book 3: {} by {}", book3.title, book3.author);
        println!("Book 4: {} by {}", book4.title, book4.author);

        let mut lib = super::Library::new(vec![book1.clone(), book2.clone(), book4.clone()]);
        lib.update_hashmaps_on_new();
        println!("--> Library::new()");
        println!("{:#?}", lib);
        println!("#################################");

        lib.add_book(&book3);
        println!("--> Library::add_book()");
        println!("{:#?}", lib);
        println!("#################################");

        lib.remove_book(&book3);
        println!("--> Library::remove_book()");
        println!("{:#?}", lib);
        println!("#################################");

        lib.give_book(&book1);
        println!("--> Library::give_book()");
        println!("{:#?}", lib);
        println!("#################################");

        lib.receive_book(&book1);
        println!("--> Library::receive_book()");
        println!("{:#?}", lib);
        println!("#################################");

        let temp = lib.find_book_by_title("The Rust Programming Language");
        println!("--> Library::find_book_by_title()");
        println!("{:#?}", temp);
        println!("#################################");

        let temp = lib.find_book_by_isbn("978-0-201-83595-3");
        println!("--> Library::find_book_by_isbn(()");
        println!("{:#?}", temp);
        println!("#################################");

        let temp = lib.find_all_books_by_author("Brian W. Kernighan");
        println!("--> Library::find_all_books_by_author(()");
        println!("{:#?}", temp);
        println!("#################################");


        let temp = lib.find_books_by_keyword("c OR go");
        println!("--> Library::find_books_by_keyword(()");
        println!("{:#?}", temp);
        println!("#################################");


        let temp = lib.find_books_by_keyword("programming");
        println!("--> Library::find_books_by_keyword(()");
        println!("{:#?}", temp);
        println!("#################################");


        let temp = lib.find_books_by_keyword("programming AND go");
        println!("--> Library::find_books_by_keyword(()");
        println!("{:#?}", temp);
        println!("#################################");


        let temp = lib.find_books_by_keyword("go OR rust AND programming");
        println!("--> Library::find_books_by_keyword(()");
        println!("{:#?}", temp);
        println!("#################################");



    }
}
