use std::fs;

pub fn introduction_to_error_handling(){
    let result = divide(6.0, 78.0);
    println!("{:?}", result );
    
    let my_content = get_file_content("my_content.txt");

    match my_content {
        Ok(item) => println!("The result is :{}", item),
        Err(_) => println!("We got an error"),
    }
}

fn get_file_content(file_name: &str) -> Result<String, std::io::Error>{
    let content = fs::read_to_string(file_name)?;
    Ok(content)
}

fn divide(numerator: f64, denominator: f64) -> Option<f64>{
    if denominator == 0.0{
        return None;
    }
    else {
        return Some(numerator/denominator);
    }
}