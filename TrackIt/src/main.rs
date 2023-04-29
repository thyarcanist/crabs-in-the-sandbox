use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
    genre: String,
    page_count: u32,
}

fn main() {
    print_banner();
    let mut books: HashSet<Book> = load_books().unwrap_or_else(|err| {
        eprintln!("Error loading books: {}", err);
        HashSet::new()
    });

    loop {
        println!("\nChoose an option:");
        println!("1. Add a book");
        println!("2. Remove a book");
        println!("3. Search for books");
        println!("4. List all books");
        println!("5. Exit");

        let choice = read_input();
        match choice.trim().parse::<u8>() {
            Ok(1) => add_book(&mut books),
            Ok(2) => remove_book(&mut books),
            Ok(3) => search_books(&books),
            Ok(4) => list_books(&books),
            Ok(5) => {
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

fn add_book(books: &mut HashSet<Book>) {
    println!("Enter the title:");
    let title = read_input().trim().to_string();

    println!("Enter the author:");
    let author = read_input().trim().to_string();

    println!("Enter the genre:");
    let genre = read_input().trim().to_string();

    println!("Enter the page count:");
    let page_count: u32 = read_input().trim().parse().unwrap_or(0);

    let book = Book {
        title,
        author,
        genre,
        page_count,
    };
    books.insert(book);
    println!("Book added successfully.");

    if let Err(err) = save_books(&books) {
        eprintln!("Error saving books: {}", err);
    }
}

fn remove_book(books: &mut HashSet<Book>) {
    println!("Enter the title of the book you want to remove:");
    let title = read_input().trim().to_string();

    if let Some(book) = books.iter().find(|b| b.title == title).cloned() {
        books.remove(&book);
        println!("Book removed successfully.");

        if let Err(err) = save_books(&books) {
            eprintln!("Error saving books: {}", err);
        }
    } else {
        println!("Book not found.");
    }
}

fn search_books(books: &HashSet<Book>) {
    println!("Search by: 1. Author, 2. Genre, 3. Page count");
    let choice = read_input();
    match choice.trim().parse::<u8>() {
        Ok(1) => {
            println!("Enter the author:");
            let author = read_input().trim().to_string();
            let filtered_books: Vec<&Book> = books.iter().filter(|b| b.author == author).collect();
            list_filtered_books(&filtered_books);
        }
        Ok(2) => {
            println!("Enter the genre:");
            let genre = read_input().trim().to_string();
            let filtered_books: Vec<&Book> = books.iter().filter(|b| b.genre == genre).collect();
            list_filtered_books(&filtered_books);
        }
        Ok(3) => {
            println!("Enter the page count:");
            let page_count: u32 = read_input().trim().parse().unwrap_or(0);
            let filtered_books: Vec<&Book> = books.iter().filter(|b| b.page_count == page_count).collect();
            list_filtered_books(&filtered_books);
        }
        _ => println!("Invalid choice. Please try again."),
    }
}

fn list_books(books: &HashSet<Book>) {
    if books.is_empty() {
        println!("No books in the dataset.");
    } else {
        println!("List of books: \n");
        for book in books {
            println!("{:?}", book);
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

fn save_books(books: &HashSet<Book>) -> Result<(), DataError> {
    let json = serde_json::to_string(books)?;
    fs::write("books.json", json)?;
    Ok(())
}

fn load_books() -> Result<HashSet<Book>, DataError> {
    if Path::new("books.json").exists() {
        let json = fs::read_to_string("books.json")?;
        let books: HashSet<Book> = serde_json::from_str(&json)?;
        Ok(books)
    } else {
        Ok(HashSet::new())
    }
}

