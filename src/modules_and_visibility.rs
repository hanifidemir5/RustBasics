use sports::football;

pub fn modules_and_visibility(){
    football();

    let messi =sports::FootballPlayer{
        name:"Messi".to_string(),
        age: 19,
    };

    println!("Name: {}, Age: {}", messi.name, messi.age );
}

mod sports {
    pub fn football(){
        println!("Let's play football... actually with out foot...")
    }

    pub struct FootballPlayer{
        pub name:String,
        pub age:i32,
    }
}