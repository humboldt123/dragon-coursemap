mod course;


use futures::future::{BoxFuture, FutureExt};
use std::collections::HashMap;
use crate::course::Course;

#[tokio::main]
async fn main() {
    let mut courses: HashMap<String, Course> = HashMap::new();
    add_course_and_prerequisites("CS 380", &mut courses).await;
    println!("{:?}", courses);
}

// I know there's a better way to do this
fn add_course_and_prerequisites<'a> (
    course_code: &'a str,
    courses: &'a mut HashMap<String, Course>,
) -> BoxFuture<'a, ()> {
    async move {
        let course = Course::new(course_code).await;
        for chunk in &course.prerequisites {
            for item in chunk {
                add_course_and_prerequisites(item, courses).await;
            }
        }
        courses.insert(course_code.to_string(), course);
    }.boxed()
}
