pub fn vectors () {
    let mut numbers = vec![1,2,3,4];
    let mut names: Vec<String> = Vec::new();

    names.push(String::from("Alice"));
    names.push(String::from("Bob"));

    let first_name = &names[0];
    let second_name = &names[1];

    println!("First name: {}, Second name: {}", first_name, second_name);

    names.pop();

    for number in numbers{
        println!("The number is {}", number);
    }

}
