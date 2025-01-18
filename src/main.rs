mod enums;
mod structs;
mod optionAndResult;

use crate::structs::{concatenate_strings, get_book_data, Book, TupleBook, UnitBook,create_book,Rectangle};
use crate::enums::{Weather, Message,process_message, Animal};

use crate::optionAndResult::{find_square_root,divide, get_from_database, calculate_triangle_area};

fn main(){
    let string1 = String::from("hello");
    let string2 = String::from(" world\n");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);

    let book = Book{
        title: String::from("The Way of Zen"),
        author: String::from("Alan Waats"),
        publication_year: 1957,
    } ;
    println!(
        "The book {} is written by {} in {}", 
        book.title, book.author, book.publication_year
    );

    let mut book = Book{
        title: String::from("The Way of Zen"),
        author: String::from("Alan Waats"),
        publication_year: 1957,
    };
    
    book.publication_year = 1989;

    println!(
        "The book {} is written by {} in {}", 
        book.title, book.author, book.publication_year
    );

    let book_data = get_book_data(book);
    for data in book_data{
        println!("{data}");
    }

    let my_book = create_book("The path of Zen".to_string(), "Simon".to_string(),1989 );

    println!("My book is {:?}", my_book);

    let tuple_book = TupleBook("The Path of the Zen".to_string(), "Simon".to_string(), 1989);
    let title = tuple_book.0;
    let author = tuple_book.1;
    let publication_year = tuple_book.2;

    println!("Title: {}, Author: {}, Publication Year: {}", title, author, publication_year);

    let my_rectangle = Rectangle{
        width: 10.0,
        height: 5.0,
    };

    let area = my_rectangle.area();

    println!("The area of the rectangle is {}", area);

    let current_weather = Weather::Snowy;

    let msg = Message::Write(String::from("Hello Rust"));

    process_message(msg);

    let my_pet = Animal::Cat("Melo".to_string());

    if let Animal::Cat(name) = my_pet{
        println!("My cat name is {}", name);
    }
    else{
        println!("My animal is not a cat");
    }
    
    let msg = Message::Write(String::from("Melo is sleeping"));
    msg.call();

    let number = -4.0;
    let square_root = find_square_root(number);

    match square_root {
        Some(value) => println!("The square root of {} is {}", number, value),
        None => println!("The square root of {} is not a real number",number),
    }

    let a = 10.0;
    let b = 0.0;
    let division_result = divide(a, b);

    match  division_result {
        Ok(value) => println!("{} divided by {} is  {}",a,b,value),
        Err(error_message) => println!("Error {}", error_message),
    }

    let base = get_from_database("base");
    let height = get_from_database("height");
    let area_result = calculate_triangle_area(base, height);

    match area_result {
        Ok(area) => println!("Area of the triangle is {} square units", area),
        Err(error_message) => println!("Error: {}", error_message)
    }

}