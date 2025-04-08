use std::collections::{HashMap, HashSet};
///
/// Livraria 2.0
///
/// Para implementar esta iteração do exercício, deve copiar a versão anterior livraria 1.0
/// E fazer todas as alterações pedidas pelo enunciado.
///
/// Devem manter ambas as versões do exercício.

/// Augmente a livraria feita anteriormente com estruturas de dados para eficientemente encontrar um livro pelo seu título ou ISBN e encontrar os livros escritos por um autor. Introduzir procura por palavras chave eficiente com a capacidade de fazer procura por interseção de palavras chave ou união de palavras chave.

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
    id: usize,
    units: usize,
}

impl Book {
    fn new(isbn: &str, title: &str, author: &str, keywords: Vec<&str>, id: usize) -> Self {
        Self {
            isbn: isbn.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            keywords: keywords.iter().map(|s| s.to_string()).collect(),
            id,
            units: 1,
        }
    }
}

#[derive(Debug)]
struct Library {
    books: HashMap<usize, Book>,
    books_by_title: HashMap<String, HashSet<usize>>,
    books_by_isbn: HashMap<String, HashSet<usize>>,
    books_by_author: HashMap<String, HashSet<usize>>,
    books_by_keyword: HashMap<String, HashSet<usize>>,
}

impl Library {
    // V2.0 - constructor
    fn new(books: Vec<Book>) -> Self {
        let mut instance = Self {
            books: HashMap::new(),
            books_by_title: HashMap::new(),
            books_by_isbn: HashMap::new(),
            books_by_author: HashMap::new(),
            books_by_keyword: HashMap::new(),
        };

        for book in books.iter() {
            instance.add_book(book);
            instance.update_hashmaps_on_add(book);
        }

        instance
    }

    // auxiliary methods
    fn list_ids(&self) -> Vec<&usize> {
        self.books.keys().collect::<Vec<&usize>>()
    }

    fn has_book_id(&self, book_id: usize) -> bool {
        self.books.contains_key(&book_id)
    }

    fn has_book(&self, book: &Book) -> bool {
        self.books.contains_key(&book.id)
    }

    fn has_book_available(&self, book: &Book) -> bool {
        if self.has_book(book) {
            self.books.get(&book.id).unwrap().units > 0
        } else {
            false
        }
    }

    // V2.0
    fn find_book_by_title(&self, title: &str) -> Book {
        let idx = self
            .books_by_title
            .get(title)
            .expect("Could not find book")
            .iter()
            .next();

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
            .next();

        if let Some(idx) = idx {
            self.books[idx].clone()
        } else {
            panic!["Found book with matching ISBN, but its index was not registered in the HashMap"]
        }
    }

    // V2.0
    fn find_books_by_author(&self, author: &str) -> Vec<Book> {
        let indices = self
            .books_by_author
            .get(author)
            .expect("Could not find book");

        let mut books = Vec::<Book>::new();
        for idx in indices {
            books.push(self.books[idx].clone());
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
        for (i, (_kw, kw_idx)) in keywords
            .into_iter()
            .zip(keyword_indices.into_iter())
            .enumerate()
        {
            if i > 0 {
                match operators[i - 1] {
                    "AND" => {
                        processed_indices = processed_indices
                            .intersection(kw_idx)
                            .copied()
                            .collect::<HashSet<_>>();
                    }
                    "OR" => {
                        processed_indices = processed_indices
                            .union(kw_idx)
                            .copied()
                            .collect::<HashSet<_>>();
                    }
                    _ => panic!["Found non-valid operator in query!"],
                }
            } else {
                processed_indices.extend(kw_idx);
            }
        }

        let mut books = Vec::<Book>::new();
        for idx in processed_indices {
            books.push(self.books[&idx].clone());
        }
        books
    }

    // required methods
    fn add_book(&mut self, book: &Book) {
        if self.has_book(book) {
            if let Some(item) = self.books.get_mut(&book.id) {
                item.units += book.units;
            }
        } else {
            self.books.insert(book.id, book.clone());
            self.update_hashmaps_on_add(book);
        }
    }

    fn update_hashmaps_on_add(&mut self, book: &Book) {
        // update HashMaps
        if self.books_by_title.contains_key(&book.title) {
            self.books_by_title
                .get_mut(&book.title)
                .unwrap()
                .insert(book.id);
        } else {
            self.books_by_title
                .insert(book.title.clone(), HashSet::new());
            self.books_by_title
                .get_mut(&book.title)
                .unwrap()
                .insert(book.id);
        }

        if self.books_by_isbn.contains_key(&book.isbn) {
            self.books_by_isbn
                .get_mut(&book.isbn)
                .unwrap()
                .insert(book.id);
        } else {
            self.books_by_isbn.insert(book.isbn.clone(), HashSet::new());
            self.books_by_isbn
                .get_mut(&book.isbn)
                .unwrap()
                .insert(book.id);
        }

        if self.books_by_author.contains_key(&book.author) {
            self.books_by_author
                .get_mut(&book.author)
                .unwrap()
                .insert(book.id);
        } else {
            self.books_by_author
                .insert(book.author.clone(), HashSet::new());
            self.books_by_author
                .get_mut(&book.author)
                .unwrap()
                .insert(book.id);
        }

        for keyword in book.keywords.iter() {
            if self.books_by_keyword.contains_key(keyword) {
                self.books_by_keyword
                    .get_mut(keyword)
                    .unwrap()
                    .insert(book.id);
            } else {
                self.books_by_keyword
                    .insert(keyword.to_string().clone(), HashSet::new());
                self.books_by_keyword
                    .get_mut(keyword)
                    .unwrap()
                    .insert(book.id);
            }
        }
    }

    fn remove_book(&mut self, book: &Book) {
        if self.has_book(book) {
            self.books.remove(&book.id);
        }
        self.update_hashmaps_on_remove(book);
        self.cleanup_on_remove();
    }

    // DONE:  the HashSets might be left empty!
    //        Need to implement cleanup method that deletes keys from HashMaps
    //        when their HashSets are empty
    //        + memory cleanup
    //        - inefficient to iterate over HashMap
    fn update_hashmaps_on_remove(&mut self, book: &Book) {
        // update HashMaps
        if self.books_by_title.contains_key(&book.title) {
            self.books_by_title
                .get_mut(&book.title)
                .unwrap()
                .remove(&book.id);
        }

        if self.books_by_isbn.contains_key(&book.isbn) {
            self.books_by_isbn
                .get_mut(&book.isbn)
                .unwrap()
                .remove(&book.id);
        }

        if self.books_by_author.contains_key(&book.author) {
            self.books_by_author
                .get_mut(&book.author)
                .unwrap()
                .remove(&book.id);
        }

        for keyword in book.keywords.iter() {
            if self.books_by_keyword.contains_key(keyword) {
                self.books_by_keyword
                    .get_mut(keyword)
                    .unwrap()
                    .remove(&book.id);
            }
        }
    }

    fn cleanup_on_remove(&mut self) {
        // e.g. empty HashSets in the auxiliary fields
        // temporary list of mutable borrows of each HashMap
        let hashmaps = vec![
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
                .filter(|(_k, v)| v.is_empty())
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
        if self.has_book_available(book) {
            if let Some(item) = self.books.get_mut(&book.id) {
                item.units -= 1;
            }
        } else {
            panic!("No book available! Library will now self destruct...")
        }
    }

    fn receive_book(&mut self, book: &Book) {
        if self.has_book(book) {
            if let Some(item) = self.books.get_mut(&book.id) {
                item.units += 1;
            }
        } else {
            // book was not registered, but will be
            self.add_book(book);
        }
    }
}

fn create_examples() -> (Book, Book, Book, Book) {
    let book1 = Book::new(
        "978-3-16-148410-0",
        "The Rust Programming Language",
        "Steve Klabnik",
        vec!["rust", "programming", "systems"],
        100,
    );

    let book2 = Book::new(
        "978-0-201-83595-3",
        "The C Programming Language",
        "Brian W. Kernighan",
        vec!["c", "programming", "classic"],
        200,
    );

    let book3 = Book::new(
        "978-1-59327-599-0",
        "Automate the Boring Stuff with Python",
        "Al Sweigart",
        vec!["python", "automation", "beginner"],
        300,
    );

    let book4 = Book::new(
        "978-0134190440",
        "The Go Programming Language",
        "Brian W. Kernighan",
        vec!["go", "programming", "concurrency", "goroutines", "channels"],
        201,
    );

    (book1, book2, book3, book4)
}

// cli stuff
fn option_menu() {
    println!("0: list IDs");
    println!("1: create new Book entry");
    println!("2: delete an existing Book entry");
    println!("3: request a Book");
    println!("4: return a Book");
    println!("5: search Book by title");
    println!("6: search Book by ISBN");
    println!("7: search Book by author");
    println!("8: search Book by keyword");
    println!("9: consult Library");
    println!("_: quit");
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

fn integer_input() -> usize {
    let input = string_input();
    input.parse::<usize>().unwrap()
}

fn unique_id_input(lib: Option<&Library>) -> usize {
    // If you call this method with a reference to the lib,
    // you are prompted repeatedly until you enter an ID which is not present in the lib
    println!("Please enter an id: ");
    let id = integer_input();
    if let Some(lib) = lib {
        if lib.has_book_id(id) {
            println!("ID already registered!");
            println!("Registered IDs: {:#?}", lib.list_ids());
            unique_id_input(Some(lib));
        }
    }
    id
}

fn book_input(lib: Option<&Library>) -> Book {
    let id = unique_id_input(lib);
    println!("Please enter your book ISBN code: ");
    let isbn = string_input();
    println!("Please enter your book title: ");
    let title = string_input();
    println!("Please enter your book author: ");
    let author = string_input();
    println!("Please enter its keywords (whitespace separated): ");
    let keywords = string_input();
    let keywords: Vec<&str> = keywords.split_whitespace().collect();
    Book::new(&isbn, &title, &author, keywords, id)
}

fn main() {
    let (book1, book2, book3, book4) = create_examples();
    let mut lib = Library::new(vec![book1, book2, book3, book4]);

    loop {
        println!("Choose an option:");
        option_menu();
        let option = option_input();

        match option {
            0 => println!("Registered IDs: {:#?}", lib.list_ids()),
            1 => {
                let book = book_input(Some(&lib));
                lib.add_book(&book)
            }
            2 => {
                let book = book_input(None);
                lib.remove_book(&book)
            }
            3 => {
                let book = book_input(None);
                lib.give_book(&book)
            }
            4 => {
                let book = book_input(None);
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
                let books = lib.find_books_by_author(key.as_str());
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

        let temp = lib.find_books_by_author("Brian W. Kernighan");
        println!("--> Library::find_all_books_by_author(()");
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

        let temp = lib.find_books_by_keyword("c OR go");
        println!("--> Library::find_books_by_keyword(()");
        println!("{:#?}", temp);
        println!("#################################");

        let temp = lib.find_books_by_keyword("go OR rust AND programming");
        println!("--> Library::find_books_by_keyword(()");
        println!("{:#?}", temp);
        println!("#################################");
    }
}
