pub fn closures() {
    let my_closure = || println!("Defining closures");

    my_closure();

    let even_numbers = |x:i32| -> bool { x % 2 == 0};

    even_numbers(4);
    even_numbers(5); 
}