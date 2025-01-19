pub fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    return result;
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub publication_year: u32,
}

pub struct  TupleBook(pub String, pub String,pub u32);

pub fn get_book_data(book:Book) -> [String; 3]{
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    data
}

pub fn create_book(title: String, author: String, publication_year: u32) -> Book{
    let book = Book{
        title,
        author,
        publication_year,
    };

    return book
}

pub struct Rectangle{
    pub width: f64,
    pub height: f64,
}

impl Rectangle{
    pub fn area(&self) -> f64{
        self.width * self.height
    }
}