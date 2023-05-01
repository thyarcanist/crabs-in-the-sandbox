use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct Book {
    title: String,
    author: String,
    genre: String,
    page_count: u32,
    finished: bool,
    pages_read: u32,
    progress_bar: String,
}

impl Default for Book {
    fn default() -> Self {
        Book {
            title: "".to_string(),
            author: "".to_string(),
            genre: "".to_string(),
            page_count: 0,
            finished: false,
            pages_read: 0,
            progress_bar: create_progress_bar(0, 0),
        }
    }
}

impl Book {
    fn new(title: &str, author: &str, genre: &str, page_count: u32, finished: bool, pages_read: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            genre: genre.to_string(),
            page_count,
            finished,
            pages_read,
            progress_bar: create_progress_bar(page_count, pages_read),
        }
    }
}

use std::hash::{Hash, Hasher};

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for Book {}

impl Hash for Book {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.title.hash(state);
    }
}



fn main() {
    print_banner();
    let mut books: HashMap<String, Book> = load_books().unwrap_or_else(|err| {
        eprintln!("Error loading books: {}", err);
        HashMap::new()
    });

    loop {
        println!("\nChoose an option:");
        println!("1. Add a book");
        println!("2. Remove a book");
        println!("3. Search for books");
        println!("4. List all books");
        println!("5. Edit a book");
        println!("6. Exit");

        let choice = read_input();
        match choice.trim().parse::<u8>() {
            Ok(1) => add_book(&mut books),
            Ok(2) => remove_book(&mut books),
            Ok(3) => search_books(&books),
            Ok(4) => list_books(&books),
            Ok(5) => edit_book(&mut books),
            Ok(6) => {
                if let Err(err) = save_books(&books) {
                    eprintln!("Error saving books: {}", err);
                }
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn print_banner() {
    println!(
        " ___________  ___  _____  _   _______ _____\n\
         |_   _| ___ \\/ _ \\/  __ \\| | / /_   _|_   _|\n\
         | | | |_/ / /_\\ \\ /  \\/| |/ /  | |   | |\n\
         | | |    /|  _  | |    |    \\  | |   | |\n\
         | | | |\\ \\| | | | \\__/\\| |\\  \\_| |_  | |\n\
         \\_/ \\_| \\_\\_| |_/\\____/\\_| \\_/\\___/  \\_/\n\
                                             \n\
                 by AgentNil"
    );
}




fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn add_book(books: &mut HashMap<String, Book>) {
    println!("Enter the title:");
    let title = read_input().trim().to_string();

    println!("Enter the author:");
    let author = read_input().trim().to_string();

    println!("Enter the genre:");
    let genre = read_input().trim().to_string();

    println!("Enter the page count:");
    let page_count: u32 = read_input().trim().parse().unwrap_or(0);

    println!("Have you finished reading this book? (y/n):");
    let finished = read_input().trim().to_lowercase() == "y";

    let pages_read = if finished {
        page_count
    } else {
        println!("Enter the number of pages read:");
        read_input().trim().parse().unwrap_or(0)
    };

    let progress_bar = create_progress_bar(page_count, pages_read);

    let book = Book {
        title: title.clone(),
        author,
        genre,
        page_count,
        finished,
        pages_read,
        progress_bar,
    };
    books.insert(title, book);
    println!("Book added successfully.");

    if let Err(err) = save_books(&books) {
        eprintln!("Error saving books: {}", err);
    }
}

fn remove_book(books: &mut HashMap<String, Book>) {
    println!("Enter the title of the book you want to remove:");
    let title = read_input().trim().to_string();

    if books.remove(&title).is_some() {
        println!("Book removed successfully.");

        if let Err(err) = save_books(&books) {
            eprintln!("Error saving books: {}", err);
        }
    } else {
        println!("Book not found.");
    }
}

fn search_books(books: &HashMap<String, Book>) {
    println!("Search by: 1. Author, 2. Genre, 3. Page count");
    let choice = read_input();
    match choice.trim().parse::<u8>() {
        Ok(1) => {
            println!("Enter the author:");
            let author = read_input().trim().to_string();
            let filtered_books: Vec<&Book> = books.values().filter(|b| b.author == author).collect();
            list_filtered_books(&filtered_books);
        }
        Ok(2) => {
            println!("Enter the genre:");
            let genre = read_input().trim().to_string();
            let filtered_books: Vec<&Book> = books.values().filter(|b| b.genre == genre).collect();
            list_filtered_books(&filtered_books);
        }
        Ok(3) => {
            println!("Enter the page count:");
            let page_count: u32 = read_input().trim().parse().unwrap_or(0);
            let filtered_books: Vec<&Book> = books.values().filter(|b| b.page_count == page_count).collect();
            list_filtered_books(&filtered_books);
        }
        _ => println!("Invalid choice. Please try again."),
    }
}

fn list_books(books: &HashMap<String, Book>) {
    if books.is_empty() {
        println!("No books found.");
    } else {
        for book in books.values() {
            println!("Title: {}", book.title);
            println!("Author: {}", book.author);
            println!("Genre: {}", book.genre);
            println!("Page count: {}", book.page_count);
            println!("Finished: {}", book.finished);
            println!("Pages read: {}", book.pages_read);

            let progress_bar = create_progress_bar(book.page_count, book.pages_read);
            println!("Progress: {}", progress_bar);

            println!();
        }
    }
}


fn list_filtered_books(filtered_books: &[&Book]) {
    if filtered_books.is_empty() {
        println!("No books found.");
    } else {
        println!("Found books:");
        for book in filtered_books {
            println!("{:?}", book);
        }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
}

fn save_books(books: &HashMap<String, Book>) -> Result<(), Box<dyn Error>> {
    let file = File::create("books.json")?;
    let books_vec: Vec<&Book> = books.values().collect();
    println!("Saving {} books", books_vec.len());
    serde_json::to_writer_pretty(file, &books_vec)?;
    Ok(())
}


fn load_books() -> Result<HashMap<String, Book>, Box<dyn Error>> {
    let file = File::open("books.json")?;
    let mut books_vec: Vec<Book> = serde_json::from_reader(file)?;

    for book in &mut books_vec {
        book.progress_bar = create_progress_bar(book.page_count, book.pages_read);
    }

    let books: HashMap<String, Book> = books_vec.into_iter().map(|book| (book.title.clone(), book)).collect();

    Ok(books)
}


fn find_book_by_title<'a>(books: &'a HashMap<String, Book>, title: &str) -> Option<&'a Book> {
    let lowercase_title = title.trim().to_lowercase();
    books
        .iter()
        .find(|(_, book)| book.title.trim().to_lowercase() == lowercase_title)
        .map(|(_, book)| book)
}







// Book Editing

fn edit_book(books: &mut HashMap<String, Book>) {
    println!("Enter the title of the book you want to edit:");
    let title = read_input();

    let books_clone = books.clone();
    let book_to_edit = find_book_by_title(&books_clone, &title);

    if let Some(book) = book_to_edit {
        let mut updated_book = book.clone();

        println!("Enter a new title or leave blank to keep the current title:");
        let new_title = read_input();
        if !new_title.trim().is_empty() {
            updated_book.title = new_title.trim().to_string();
        }

        println!("Enter a new author or leave blank to keep the current author:");
        let new_author = read_input();
        if !new_author.trim().is_empty() {
            updated_book.author = new_author.trim().to_string();
        }

        println!("Enter a different genre or leave blank to keep the current genre:");
        let new_genre = read_input();
        if !new_genre.trim().is_empty() {
            updated_book.genre = new_genre.trim().to_string();
        }

        println!("Enter your most recent page number or leave blank to keep the current page number:");
        let new_page_count = read_input();
        if !new_page_count.trim().is_empty() {
            match new_page_count.trim().parse::<u32>() {
                Ok(parsed_page_count) => updated_book.page_count = parsed_page_count,
                Err(_) => println!("Invalid page count. Keeping the current page count."),
            }
        }

        // Add input prompt for updating pages read
        println!("Enter the number of pages read or leave blank to keep the current pages read:");
        let new_pages_read = read_input();
        if !new_pages_read.trim().is_empty() {
            match new_pages_read.trim().parse::<u32>() {
                Ok(parsed_pages_read) => updated_book.pages_read = parsed_pages_read,
                Err(_) => println!("Invalid pages read. Keeping the current pages read."),
            }
        }

        // Remove the old book and insert the updated one
        books.remove(&book.title);
        books.insert(updated_book.title.clone(), updated_book);
    } else {
        println!("No book found with the provided title.");
    }
}


fn create_progress_bar(page_count: u32, pages_read: u32) -> String {
    let percentage = (pages_read as f64 / page_count as f64) * 10.0;
    let progress = percentage.floor() as usize;
    let bar = "=".repeat(progress) + &" ".repeat(10 - progress);
    format!("[{}]", bar)
}



