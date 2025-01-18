use std::os::unix::net::UnixDatagram;

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

    let unit_book = UnitBook;

    let my_rectangle = Rectangle{
        width: 10.0,
        height: 5.0,
    };

    let area = my_rectangle.area();

    println!("The area of the rectangle is {}", area);

}

fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    return result;
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

struct  TupleBook(String, String, u32);

struct UnitBook;
fn get_book_data(book:Book) -> [String; 3]{
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    data
}

fn create_book(title: String, author: String, publication_year: u32) -> Book{
    let book = Book{
        title,
        author,
        publication_year,
    };

    return book
}

struct Rectangle{
    width: f64,
    height: f64,
}

impl Rectangle{
    fn area(&self) -> f64{
        self.width * self.height
    }
}