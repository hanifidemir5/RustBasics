pub fn lifetimes(){
    let name = "Elon Must";
    let person = Person{
        name: &name
    };

    println!("Name: {}", person.name);
    let s1 = String::from("Pneumonoultramicroscopicsilicovolcanoconiosis");
    let s2 = String::from("Hippopotomonstrosesquippedaliophobia");
    println!("Longest word is: {}", longest(&s1, &s2));
    let sentence = Sentence{
        content: "hello world",
    };
    println!("Yell: {}, content: {}", sentence.yell(), sentence.content);

    let s: &'static str = "dummy text";

    static something : &str = "dummy text";

}

fn longest<'a   >(s1: &'a str, s2:&'a str) -> &'a str{
    if s1.len() > s2.len(){
        return s1;
    } else{
        return s2;
    }
}

struct Sentence<'a>{
    content: &'a str,
}

impl<'a>Sentence<'a>{
    fn yell(&self) -> &str{
        return "Do not code until 3AM...";
    }
}

struct Person <'a> {
    name: &'a str,
}