pub fn find_square_root(number:f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())
    } else {
        None
    }
}

pub fn divide(a:f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero is not allowed".to_string())
    }
    else {
        Ok(a/b)
    }
}

pub fn get_from_database(key:&str) -> Option<f64> {
    let database : [(&str, Option<f64>); 2] = [("base", Some(4.0)), ("height", Some(6.0))];

    for(k,v) in database{
        if k == key {
            return v;
        }
    }
    None
}

pub fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String> {
    match (base,height) {
        (Some(base), Some(height)) => {
            if base <= 0.0 || height <= 0.0 {
                Err("Both base and height must be positive numbers".to_string())
            }
            else {
                Ok(0.5 * base * height)
            }
    }
    (None, _) => Err("The base is missing".to_string()),
    (_, None) => Err("The height is missing".to_string()),
    }
}