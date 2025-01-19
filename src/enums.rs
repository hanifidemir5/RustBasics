use std::result;

pub enum Weather{
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

pub enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub enum Animal{
    Dog(String),
    Cat(String),
    Parrot(String),
}

pub fn process_message(msg:Message){
     match msg  {
        Message::Quit =>{
            println!("The Quit vaiant has no data.");
        }
        Message::Move { x, y } => {
            println!("Move to coordinates x: {} y: {}",x,y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r,g,b ) => {
            println!("Change the color to red: {}, green:{}, blue:{}",r,g,b);
        }
     }
}

impl Message {
    pub fn call(&self){
        match self{
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x:{}, y:{}", x,y),
            Message::Write(s) => println!("Write: {}",s),
            Message::ChangeColor(r,g,b) => println!("Change color to red:{}, green:{}, blue:{}",r,g,b),
        }
    }
}

pub enum Operation {
    Add{a: f64,b:f64},
    Subtract{a: f64,b:f64},
    Multiply{a: f64,b:f64},
    Divide{a: f64,b:f64},
}

pub fn calculate (operation:Operation) -> f64 {
    let result = match operation {
        Operation::Add { a, b } => a + b,
        Operation::Subtract { a, b } => a - b,
        Operation::Multiply { a, b } => a * b,
        Operation::Divide { a, b } => a / b,
    };
    result
}