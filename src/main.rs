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
mod custom_filter;
mod traits;
mod trait_objects;
mod introduction_to_generics;
mod implementation_of_generics;
mod lifetimes;
mod traits_and_generics_homework;
mod panic;
use std::io;

use crate::panic::panic;
use crate::traits_and_generics_homework::traits_and_generics_homework;
use crate::lifetimes::lifetimes;
use crate::implementation_of_generics::implementations_of_generics;
use crate::introduction_to_generics::introduction_to_generics;
use crate::trait_objects::trait_objects;
use crate::traits::traits;
use crate::custom_filter::custom_filter_file;
use crate::modules_and_visibility::modules_and_visibility;
use crate::closures::closures;
use crate::usage_of_iterators::usage_of_iterators;
use crate::iterators::{Fibonacci, iterators};
use crate::strings::strings;
use crate::vectors::vectors;
use crate::structs::{concatenate_strings, get_book_data, Book, TupleBook,create_book,Rectangle};
use crate::enums::{ Message,process_message, Animal, calculate, Operation};
use crate::hashmaps::hashmaps;
use crate::option_and_result::{find_square_root,divide, get_from_database, calculate_triangle_area};

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

    hashmaps();
    vectors();
    strings();

    let mut input = String::new();
    println!("Please enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    // Clear the input buffer
    input.clear();

    // Reading the second number
    println!("Please enter an operation Add, Subtract, Multiply or Divide");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = input.trim().to_string();
    // Clear the input buffer
    input.clear();

    // Reading the operation
    println!("Please enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");

    let operation_enum = match operation.as_str() {
        "Add" | "add" => Some(Operation::Add { a: num1, b: num2 }),
        "Subtract" | "subtract" => Some(Operation::Subtract { a: num1, b: num2 }),
        "Multiply" | "multiply" => Some(Operation::Multiply { a: num1, b: num2 }),
        "Divide" | "divide"=> Some(Operation::Divide { a: num1, b: num2 }),
        _ => None,
    };

    if let Some(op) = operation_enum{
        let result = calculate(op);
        println!("result:{:?}", result);
    }
    else {
        println!("invalid operation")
    }
    
    input.clear();

    let mut fibonacci = Fibonacci {current:0 , next:1};

    for _ in 0..10 {
        println!("{}", fibonacci.next().unwrap())
    }

    iterators();
    usage_of_iterators();
    closures();
    modules_and_visibility();
    custom_filter_file();
    traits();
    trait_objects();
    introduction_to_generics();
    implementations_of_generics();
    lifetimes();
    traits_and_generics_homework();
    panic();
}