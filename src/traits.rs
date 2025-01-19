pub fn traits(){

 let dog = Dog{
    name:"Rudolf".to_string(),
 };

 dog.speak();
}

trait Speak {
    fn speak(&self);
}

struct Dog{
    name:String,
}

impl Speak for Dog{
    fn speak(&self) {
        println!("{} says: Woof", self.name)
    }
}