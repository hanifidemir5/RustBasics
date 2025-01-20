pub fn concatenate_strings_homework(){
    let string1 = String::from("hello");
    let string2 = String::from(" world\n");

    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}

pub fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    return result;
}
