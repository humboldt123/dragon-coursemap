mod course;

use crate::course::Course;

#[tokio::main]
async fn main() {
    let course = Course::new("CS 383").await;
    println!("{:?}", course);
}
