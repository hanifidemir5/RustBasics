pub fn panic(){
    let my_vec = vec![1,2];
    let my_variable = my_vec[5];

    let veggies = ["carrot","tomato","potato"];

    chooseVeggie(veggies[0]);
    chooseVeggie(veggies[1]);
    chooseVeggie(veggies[2]);
    chooseVeggie("egg plant");

}

fn chooseVeggie(veggie: &str){
    match veggie{
        "carrot" => println!("carrot"),
        "tomato" => println!("tomato"),
        "potato" => println!("potato"),
        _ => panic!("this is not acceptable veggie!!"),
    }
}