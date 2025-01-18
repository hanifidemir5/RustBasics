pub fn strings()
{
    let mut my_string = String::from("my");
    let mut second_string = "second string".to_string();

    my_string.push_str("string");

    println!("{}", my_string);

    for c in my_string.chars() {
        println!("{}", c);
    }

    for b in my_string.bytes() {
        println!("{}", b);
    }
}