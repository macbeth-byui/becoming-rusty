use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader, Read, Error, ErrorKind, Write};
use std::fmt;

#[derive(Deserialize, Debug)]
struct Config {
    pub server : String,
    pub token : String
}

#[derive(Deserialize)]
struct Course {
    pub id : u32,
    pub course_code : String,
    pub concluded : bool,
    pub term : CourseTerm,
    pub needs_grading_count : Option<u32>,
    pub total_students : Option<u32>,
}

#[derive(Deserialize)]
struct CourseTerm {
    pub name : String
}

#[derive(Deserialize)]
struct Student {
    pub id : u32,
    pub name : String,
    pub enrollments : Vec<Enrollment>,
}

#[derive(Deserialize)]
struct Enrollment {
    pub grades : Grades
}

#[derive(Deserialize)]
struct Grades {
    pub current_grade : Option<String>,
    pub current_score : Option<f32>
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {} - {} [Grading: {} Students: {}]",
            self.id,
            self.course_code,
            self.term.name,
            self.needs_grading_count.unwrap_or(0),
            self.total_students.unwrap_or(0)
        )
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(enrollment) = self.enrollments.first() {
            return write!(f, "{} : {} - {:?}% ({:?})",
                self.id,
                self.name,
                enrollment.grades.current_score.unwrap_or(0.0),
                enrollment.grades.current_grade.to_owned().unwrap_or("-".to_string())
            );

        }
        write!(f, "{} : {}",self.id,self.name)
    }
}

fn load_config() -> io::Result<Config> {
    let file = File::open("lms.toml")?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer)?;
    let contents = String::from_utf8(buffer)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    toml::from_str(&contents)
        .map_err(|e| Error::new(ErrorKind::Other, e))    
}

fn json_api_get<T>(config : &Config, rel_url : &str) -> io::Result<Vec<T>> 
    where T : DeserializeOwned
{
    // println!("{}",rel_url);
    let client = reqwest::blocking::Client::new();
    let mut page = 1;
    let mut all_results = Vec::<T>::new();

    loop {
        let url = format!("{}/{}&page={}&per_page=100",config.server, rel_url, page);
        let res = client.get(url)
            .header("Authorization", format!("Bearer {}", config.token))
            .send()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
            
        let headers = res.headers().clone();
        // Note that text() consumes the response
        let data = res.text()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        // println!("{}", data);
        let results: Vec<T> = serde_json::from_str(&data)?;
        all_results.extend(results);
        match headers.get("link") {
            Some(value) => {
                if value.to_str()
                    .map_err(|e| Error::new(ErrorKind::Other, e))?
                    .contains("rel=\"next\"") {
                        page += 1;
                    // println!("{:?}",value);
                }
                else {
                    break
                }
            },
            None => break
        }
    }
    Ok(all_results)
}

fn print_vec<T>(vec : &Vec<T>) where T : fmt::Display {
    for item in vec {
        println!("{}", item);
    }
}

fn convert_term(term : &str) -> String {
    let mut parts = term.split_whitespace();
    let season = parts.next().unwrap_or("");
    let year = parts.next().unwrap_or("");
    let period = match season {
        "Winter" => "1W",
        "Spring" => "2S",
        "Summer" => "3M",
        "Fall" => "4F",
        _ => return String::new()
    };
    format!("{}-{}",year,period)
}

fn get_courses(config : &Config, current : bool) -> io::Result<Vec<Course>> {
    let mut courses = json_api_get::<Course>(config, 
        "/api/v1/courses\
                 ?enrollment_type=teacher\
                 &state=available\
                 &include[]=concluded\
                 &include[]=term\
                 &include[]=needs_grading_count\
                 &include[]=total_students")?;
    courses.iter_mut().for_each(|x| x.term.name = convert_term(&x.term.name));
    courses.retain(|x| !x.term.name.is_empty());
    if current {
        courses.retain(|x| !x.concluded);
    }
    courses.sort_by(|x, y| x.course_code.cmp(&y.course_code));
    Ok(courses)
}

fn get_students(config : &Config, course : &str) -> io::Result<Vec<Student>> {
    let mut students = json_api_get::<Student>(config,&format!(
                        "/api/v1/courses/{}/users\
                         ?enrollment_type[]=student\
                         &include[]=total_scores\
                         &include[]=enrollments\
                         &enrollment_state[]=active\
                         &enrollment_state[]=invited\
                         &enrollment_state[]=completed", course))?;
    students.sort_by(|x, y| x.name.cmp(&y.name));
    Ok(students)
}

fn shell(config : Config) {
    println!();
    println!("       \\ | /     ");
    println!("     \\ ***** /   ");
    println!("    \\ * ~~~ * /  ");
    println!("  ---*-------*---");
    println!("      Horizon    ");
    println!(); 

    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush output.");
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
        let parsed = buffer
            .split_whitespace()
            .collect::<Vec<&str>>();
        if let Some(command) = parsed.first() {
            match *command {
                "courses" => {
                    if let Some(scope) = parsed.get(1) {
                        if *scope == "all" {
                            match get_courses(&config, false) {
                                Ok(courses) => print_vec(&courses),
                                Err(e) => println!("Error: {}",e)
                            }
                        }
                        else {
                            println!("Invalid Command");
                        }
                    }
                    else {
                        match get_courses(&config, true) {
                            Ok(courses) => print_vec(&courses),
                            Err(e) => println!("Error: {}",e)
                        }
                    }
                }
                "students" => {
                    if let Some(course) = parsed.get(1) {
                        match get_students(&config, course) {
                            Ok(students) => print_vec(&students),
                            Err(e) => println!("Error: {}", e)
                        }
                    }
                    else {
                        println!("Invalid Command");
                    }
                }
                "exit" => break,
                _ => println!("Invalid Command")
            }

        }
        println!();
    }
}

fn init() -> io::Result<Config> {
    load_config()
}

fn main() {
    match init() {
        Ok(config) => shell(config),
        Err(e) => println!("{}", e)
    }
    println!("Goodbye!");
    println!();
}
