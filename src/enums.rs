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
            Message:: Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x:{}, y:{}", x,y),
            Message:: Write(s) => println!("Write: {}",s),
            Message::ChangeColor(r,g,b) => println!("Change color to red:{}, green:{}, blue:{}",r,g,b),
        }
    }
}