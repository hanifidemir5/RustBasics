mod enums;
mod structs;
mod option_and_result;
mod hashmaps;
mod vectors;
mod strings;
mod iterators;
mod usage_of_iterators;
mod closures;
mod modules_and_visibility;
mod traits;
mod trait_objects;
mod introduction_to_generics;
mod lifetimes;
mod panic;
mod introduction_to_error_handling;
mod cutom_error;
mod homeworks{
    pub mod traits_and_generics_homework;
    pub mod implementations_of_generics_homework;
    pub mod enums_homework;
    pub mod custom_filter;
    pub mod concatenate_strings_homework;
}

use crate::homeworks::concatenate_strings_homework::concatenate_strings_homework;
use crate::homeworks::enums_homework::enums_homework;
use crate::cutom_error::custom_error;
use crate::introduction_to_error_handling::introduction_to_error_handling;
// use crate::panic::panic;
use crate::homeworks::traits_and_generics_homework::traits_and_generics_homework;
use crate::lifetimes::lifetimes;
use crate::homeworks::implementations_of_generics_homework::implementations_of_generics_homework;
use crate::introduction_to_generics::introduction_to_generics;
use crate::trait_objects::trait_objects;
use crate::traits::traits;
use crate::homeworks::custom_filter::custom_filter_file;
use crate::modules_and_visibility::modules_and_visibility;
use crate::closures::closures;
use crate::usage_of_iterators::usage_of_iterators;
use crate::iterators::{Fibonacci, iterators};
use crate::strings::strings;
use crate::vectors::vectors;
use crate::structs::{get_book_data, Book, TupleBook,create_book,Rectangle};
use crate::enums::{ Message,process_message, Animal};
use crate::hashmaps::hashmaps;
use crate::option_and_result::{find_square_root,divide, get_from_database, calculate_triangle_area};

fn main(){

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

    // let current_weather = Weather::Snowy;

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

    let mut fibonacci = Fibonacci {current:0 , next:1};

    for _ in 0..10 {
        println!("{}", fibonacci.next().unwrap())
    }

    concatenate_strings_homework();
    hashmaps();
    vectors();
    strings();
    enums_homework();
    iterators();
    usage_of_iterators();
    closures();
    modules_and_visibility();
    custom_filter_file();
    traits();
    trait_objects();
    introduction_to_generics();
    implementations_of_generics_homework();
    lifetimes();
    traits_and_generics_homework();
    // panic();
    introduction_to_error_handling();
    custom_error();

}