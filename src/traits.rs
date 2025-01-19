pub fn traits(){

 let dog = Dog{
    name:"Rudolf".to_string(),
 };

 dog.speak();

 let cow = Cow{
    name:"Rudolf".to_string(),
 };

 cow.speak();

 let original_string = String::from("this is original");
 let copy_string = original_string.display();

 println!("{}", copy_string);

 animal_speaks(&cow);
 animal_speaks(&dog);

 let cat = Cat;
 cat.make_sound();
 cat.walk();
 cat.sleep();
}

trait Speak {
    fn speak(&self);
}

struct Dog{
    name:String,
}

struct Cow{
    name:String,
}

impl Speak for Dog{
    fn speak(&self) {
        println!("{} says: Woof", self.name)
    }
}

impl Speak for Cow{
    fn speak(&self) {
        println!("{} says: Mooo!", self.name)
    }
}

trait Display {
    fn display(&self)-> String;
}

impl Display for String {
    fn display(&self)-> String {
        self.clone()
    }
}

fn animal_speaks<T:Speak>(animal:&T){
    animal.speak();
}

trait Animal {
    fn make_sound(&self);

    fn sleep(&self){
        println!("Animal is sleeping.");
    }
}

trait Mammal: Animal {
    fn walk(&self);
}

trait Bird: Animal {
    fn fly(&self);
}

struct Cat;

impl Animal for Cat{
    fn make_sound(&self) {
        println!("Miyav!");
    }
}

impl Mammal for Cat {
    fn walk(&self) {
        println!("Cat is walking..");
    }
}