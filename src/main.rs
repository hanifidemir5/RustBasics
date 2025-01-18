fn main(){
    let string1 = String::from("hello");
    let string2 = String::from(" world\n");

    let concatenated_string = concatenate_strings(&string1, &string2);
    print!("{}", concatenated_string);
}

fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    return result;
}
