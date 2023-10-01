mod course;

use std::collections::HashMap;
use crate::course::Course;

#[tokio::main]
async fn main() {
    let mut courses: HashMap<String, Course> = HashMap::new();
    let course = Course::new("CS 383").await;
    
    println!("{:?}", course);
}
