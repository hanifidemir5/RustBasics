use std::{collections::HashMap};
pub fn hashmaps(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);

    let alice_score = scores.get(&String::from("Alice"));
    println!("Alice's score: {:?}", alice_score);
    println!("{:?}", scores);

    scores.remove(&String::from("Alice"));

    println!("{:?}", scores);

    for (key,value) in &scores{
        println!("{} {}", key, value);
    }
}