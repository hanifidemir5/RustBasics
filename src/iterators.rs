pub struct Fibonacci {
    pub current: i32,
    pub next:i32
}

impl Iterator for Fibonacci{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item>{
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

pub fn iterators (){
    let mut vec = vec![1,2,3,4,5];
    for item in vec.iter_mut(){
        *item *= 2;
        println!("{}", item);
    }
    
    for item in vec.into_iter(){
        println!("{}", item)
    }
}

