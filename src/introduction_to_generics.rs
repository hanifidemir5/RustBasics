pub fn introduction_to_generics(){
    
}

fn swap<T: Copy>(x: &mut T,y: &mut T){
    let temp = *x;
    *x = *y;
    *y = temp;
}   

trait Summarry {
    fn summarize(&self) -> String;
}

fn print_summary<T: Summarry>(item: T){
    println!("{}", item.summarize())
}

fn print_double_summary<T, U>(item1:T, item2:U)
where 
    T:Summarry,
    U:Summarry + Clone,
{
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
    let cloned_item = item2.clone();
    println!("{}", cloned_item.summarize());
}
