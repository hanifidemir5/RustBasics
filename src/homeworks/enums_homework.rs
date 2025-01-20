use std::io;


pub fn enums_homework(){
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

pub enum Operation {
    Add{a: f64,b:f64},
    Subtract{a: f64,b:f64},
    Multiply{a: f64,b:f64},
    Divide{a: f64,b:f64},
}