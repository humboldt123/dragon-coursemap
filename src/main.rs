use reqwest;
use regex::Regex;

#[tokio::main]
async fn main() {
    let coursedata = get_raw_coursedata("CS 383").await.unwrap().to_owned();
    let y = parse_prerequisites(pull_raw_prerequisite_data(&coursedata).unwrap().as_str());
    println!("{:?}", y);
    
}

async fn get_raw_coursedata(course: &str) -> Result<String, reqwest::Error> {
    let base_url = "https://catalog.drexel.edu/ribbit/index.cgi?page=getcourse.rjs&code=".to_string();
    reqwest::get(base_url + course).await.unwrap().text().await
}


#[macro_export]
macro_rules! pull_course_data {
    ($regex:expr, $coursedata:tt) => {{
        let pattern = Regex::new($regex).unwrap();
        if let Some(captures) = pattern.captures($coursedata) {
            let content = captures.get(1).unwrap().as_str();
            Ok(content.to_string())
        } else {
            panic!("Failed to parse out coursedata with: {}", $regex);
        }
    }};
}

fn pull_name(coursedata: &str) -> Result<String, core::fmt::Error> {
    pull_course_data!(r#"</span><span class='cdspacing'>(.*?)</span>"#, coursedata)
}

fn pull_description(coursedata: &str) -> Result<String, core::fmt::Error> {
    pull_course_data!(r#"<p class="courseblockdesc">\n(.*?)<br />\n"#, coursedata)
}

fn pull_restrictions(coursedata: &str) -> Result<String, core::fmt::Error> {
    pull_course_data!(r#"<b>Restrictions:</b> (.*?)<br/>\n"#, coursedata)
}

fn pull_raw_prerequisite_data(coursedata: &str) -> Result<String, core::fmt::Error>{
    pull_course_data!(r#"<b>Prerequisites:</b> (.*?)\n<br/><br/></div>"#, coursedata)
}

fn parse_prerequisites(raw_prerequisites: &str) -> Vec<Vec<String>> {
    let pattern = Regex::new(r#"\s\[.*?\]"#).unwrap();
    let cleaned = pattern.replace_all(raw_prerequisites, "");

    let mut prerequisites: Vec<Vec<String>> = vec![];
    for item in cleaned.split(" and ").collect::<Vec<&str>>() {
        // If it contains a bracket, the student pick the prerequisite from the list. ie; (MATH 221 or MATH 222)
        if item.contains('(') {
            let mut picky_prerequisite: Vec<String> = vec![];
            for course in item[1..item.len() - 1].split(" or ").collect::<Vec<&str>>() {
                picky_prerequisite.push(course.to_string());
            }
            prerequisites.push(picky_prerequisite)
        } else {
            prerequisites.push(vec![item.to_string()]);
        }
    }
    prerequisites
}


struct _Course {
    code: String,
    name: String,
    description: String,

}