use reqwest;
use regex::Regex;

#[derive(Debug)]
pub struct Course {
    code: String,
    name: String,
    description: String,
    prerequisites: Vec<Vec<String>>
}


impl Course {
    pub async fn new(code: &str) -> Course {
        let coursedata = Self::get_raw_coursedata(code).await.unwrap();
        let raw_prerequisite_data = Self::pull_coursedata(&coursedata, r#"<b>Prerequisites:</b> (.*?)\n<br/><br/></div>"#);
        
        // todo: error handling
        Course {
            code: code.to_string(),
            name: Self::pull_coursedata(&coursedata, r#"</span><span class='cdspacing'>(.*?)</span>"#),
            description: Self::pull_coursedata(&coursedata, r#"<p class="courseblockdesc">\n(.*?)<br />\n"#),
            prerequisites: Self::parse_prerequisites(&raw_prerequisite_data)
        }
    }

    async fn get_raw_coursedata(course: &str) -> Result<String, reqwest::Error> {
        let base_url = "https://catalog.drexel.edu/ribbit/index.cgi?page=getcourse.rjs&code=".to_string();
        reqwest::get(base_url + course).await.unwrap().text().await
    }

    fn pull_coursedata(coursedata: &str, regex: &str) -> String {
        let pattern = Regex::new(regex).unwrap();
        if let Some(captures) = pattern.captures(coursedata) {
            let content = captures.get(1).unwrap().as_str();
            content.to_string()
        } else {
            panic!("Failed to parse out coursedata with: {}", regex);
        }
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
}