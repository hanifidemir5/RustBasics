
pub fn trait_objects(){
    let my_trait_object: Box<dyn MakeNoise> = Box::new(Bird{
        name:"birb".to_string(),
        color:"red".to_string(), 
    });
    
    my_trait_object.sing();

    let mut speakers: Vec<Box<dyn MakeNoise>> = Vec::new();
    speakers.push(Box::new(Bird{name:"birb2".to_string(), color:"blue".to_string()}));
    speakers.push(Box::new(Dog{name:"pluto".to_string(), breed:"kangal".to_string()}));

    for speaker in speakers{
        speaker.sing();
        invite_to_animal_the_voice(speaker);
    }

}

trait MakeNoise {
    fn sing(&self);
}

struct Bird{
    name:String,
    color:String,
}

struct Dog{
    name: String,
    breed: String,
}

impl MakeNoise for Dog{
    fn sing(&self) {
        println!("Name: {}, Breed:{}", self.name, self.breed);
        println!("Song: Bingo");
    }
}

impl MakeNoise for Bird{
    fn sing(&self) {
        println!("Name: {}, Color:{}", self.name, self.color);
        println!("Song: Diamonds");
    }
}

fn invite_to_animal_the_voice(speaker:Box<dyn MakeNoise>){
    println!("Ladies and gentelman, please welcome to our next singer: ");
    speaker.sing();
}
