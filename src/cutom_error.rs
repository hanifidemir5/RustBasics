use std::fs;

pub fn custom_error(){
    let my_error = RocketError::AlienInvasion;

    handle_error(my_error);

    match read_and_parse("text.txt") {
        Ok(num) => println!("There is nothing wrong in here {}", num),
        Err(err) => println!("An error has occured {}", err),
    }
}

enum RocketError{
    OutOfFuel,
    NavigationSystemFailure,
    AlienInvasion,
}

fn handle_error(error: RocketError){
    match error {
        RocketError::OutOfFuel =>{
            println!("out of fuel error")
        }
        RocketError::NavigationSystemFailure => {
            println!("navigation system failure")
        }
        RocketError::AlienInvasion =>{
            println!("alien invasion error")
        }
    }
}

fn divide(numerator:f64, denominator:f64) -> Result<f64, &'static str>{
    if denominator == 0.0{
        Err("you cannot divide by zero")
    }
    else {
        Ok(numerator/denominator)
    }
}

enum MyCustomError{
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Other(String),
}

use std::fmt;

impl fmt::Display for MyCustomError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self {
            MyCustomError::Io(err) => write!(f,"I/O error: {} ", err),
            MyCustomError::Parse(err) => write!(f,"Parse error: {}",err),
            MyCustomError::Other(message) => write!(f,"Other error: {}", message),
        }
    }
}

fn read_and_parse(filename: &str) ->Result<i32, MyCustomError>{
    let content = fs::read_to_string(filename).map_err(MyCustomError::Io)?;
    let num: i32 = content.trim().parse().map_err(MyCustomError::Parse)?;
    Ok(num)
}