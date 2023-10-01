use reqwest;
use regex::Regex;

#[derive(Debug)]
pub struct Course {
    pub code: String,
    pub name: String,
    pub description: String,
    pub prerequisites: Vec<Vec<String>>
}


impl Course {
    pub async fn new(code: &str) -> Course {
        let coursedata = Self::get_raw_coursedata(code).await.unwrap();
        let mut prerequisites: Vec<Vec<String>> = vec![];

        match Self::pull_coursedata(&coursedata, r#"</span><span class='cdspacing'>(.*?)</span>"#) {
            Ok(value) => {
                match Self::pull_coursedata(&coursedata, r#"<b>Prerequisites:</b> (.*?)\n<br/><br/></div>"#) {
                    Ok(value) => prerequisites = Self::parse_prerequisites(&value),
                    _ => (), // if not leave prerequisites empty
                }
        
                Course {
                    code: code.to_string(),
                    name: value,
                    description: Self::pull_coursedata(&coursedata, r#"<p class="courseblockdesc">\n(.*?)<br />\n"#).unwrap_or("No course description provided.".to_string()),
                    prerequisites: prerequisites
                }
            },
            Err(error) => {
                Course {
                    code: code.to_string(),
                    name: code.to_string(),
                    description: error,
                    prerequisites: prerequisites,
                }
            }
        }

        // If this course has prerequisites, add them to our arr
    }

    async fn get_raw_coursedata(course: &str) -> Result<String, reqwest::Error> {
        let base_url = "https://catalog.drexel.edu/ribbit/index.cgi?page=getcourse.rjs&code=".to_string();
        reqwest::get(base_url + course).await.unwrap().text().await
    }

    fn pull_coursedata(coursedata: &str, regex: &str) -> Result<String, String>{
        let pattern = Regex::new(regex).unwrap();
        if let Some(captures) = pattern.captures(coursedata) {
            let content = captures.get(1).unwrap().as_str();
            Ok(content.to_string())
        } else {
            Err(format!("Failed to parse course data.\nPattern: {}", regex))
        }
    }

    fn parse_prerequisites(raw_prerequisites: &str) -> Vec<Vec<String>> {
        // Find and remove the contents of square bracket pairs (including the brackets) plus the whitespace preceeding it
        // eg; "COURSE 101 [Min Grade: C] and COURSE 102 [Min Grade: B]" becomes "COURSE 101 and COURSE 102"
        let pattern = Regex::new(r#"\s\[.*?\]"#).unwrap();
        let cleaned = pattern.replace_all(raw_prerequisites, "");
    
        let mut prerequisites: Vec<Vec<String>> = vec![];
        for item in cleaned.split(" and ").collect::<Vec<&str>>() {
            if item.contains(" or ") {
                // If there are multiple courses, the student just has to fufill one of the prerequisites from the list.
                // ie; (MATH 221 or MATH 222)
                let mut selectable_prerequisites: Vec<String> = vec![];

                // get rid of the brackets with substring if they exist
                let cleaned = if item.contains('(') {&item[1..item.len() - 1]} else {item};
                
                for course in cleaned.split(" or ").collect::<Vec<&str>>() {
                    selectable_prerequisites.push(course.to_string());
                }
                prerequisites.push(selectable_prerequisites)
                

            } else {
                prerequisites.push(vec![item.to_string()]);
            }
        }
        prerequisites
    }
}