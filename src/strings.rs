pub fn strings()
{
    let mut my_string = String::from("my");

    my_string.push_str("string");

    println!("{}", my_string);

    for c in my_string.chars() {
        println!("{}", c);
    }

    for b in my_string.bytes() {
        println!("{}", b);
    }
}