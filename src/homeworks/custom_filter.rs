pub struct FilterCondition <T>{
    pub condition: T,
}

impl<T: PartialEq> FilterCondition<T> {
    pub fn is_match(&self, item: &T) -> bool {
        &self.condition == item
    }
}

pub fn custom_filter<T: PartialEq>(collection:Vec<T>, filter_condition: &FilterCondition<T>) -> Vec<T>{
    collection.into_iter().filter(|item| filter_condition.is_match(item)).collect()
}

pub fn custom_filter_file (){
    let vector:Vec<String> = vec!["1".to_string(), "mahmut".to_string(), "3".to_string(), "2".to_string(), "5".to_string()];
    let searched_variable = FilterCondition {
        condition : "2".to_string(),
    };

    let result = custom_filter(vector, &searched_variable );

    println!("Result is: {:?}", result);
} 