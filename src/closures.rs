pub fn closures() {
    let my_closure = || println!("Defining closures");

    my_closure();

    let even_numbers = |x:i32| -> bool { x % 2 == 0};

    let even = even_numbers(4);
    let odd = even_numbers(5); 

    println!("Even :{}, Odd: {}", even, odd);

    let numbers = vec![1,2,3,4];

    let even_numbers:Vec<i32> = numbers.into_iter().filter(|x| *x % 2 == 0).collect();

    println!("{:?}", even_numbers);

    let print_data = |data: &str|{
        println!("Received data: {}", data);
    };

    download_data("patika.dev", print_data);
}

fn download_data(url:&str, callback:impl FnOnce(&str)){
    println!("Getting data from {}", url);

    std::thread::sleep(std::time::Duration::from_secs(1));

    let data = format!("Some data from {}", url);

    callback(&data);
}