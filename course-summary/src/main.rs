#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Error: {0}")]
    Error(String),
}

#[derive(serde::Deserialize, Debug)]
pub struct Course {
    pub id: u32,
    pub name: String,
    pub total_students: Option<u32>,
}

#[derive(serde::Deserialize, Debug)]
pub struct AssignmentGroup {
    pub id: u32,
    pub name: String,
    pub position: u32,
    pub group_weight: Option<f64>,
    pub assignments: Option<Vec<Assignment>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Assignment {
    pub id: u32,
    pub name: String,
    pub assignment_group_id: u32,
    pub published: bool,
    pub points_possible: Option<f64>,
    pub allowed_attempts: i32,
    pub needs_grading_count: Option<u32>,
    pub group_category_id: Option<u32>,
    pub grade_group_students_individually: bool,
    pub html_url: String,
    pub quiz_id: Option<u32>,
    pub is_quiz_lti_assignment: Option<bool>,
    pub due_at : Option<chrono::DateTime<chrono::Utc>>,
    pub unlock_at: Option<chrono::DateTime<chrono::Utc>>,
    pub lock_at: Option<chrono::DateTime<chrono::Utc>>, 
}

#[derive(serde::Deserialize, Debug)]
pub struct Submission {
    pub id: u32,
    pub user_id: u32,
    pub attempt: Option<u32>,
    pub grade_matches_current_submission: bool,
    pub score: Option<f64>,
    pub late : bool,   
    pub excused : Option<bool>,
    pub missing : bool,
    pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub graded_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn send_canvas_request(url : &str) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let full_url = format!("https://byui.instructure.com/api/v1/{}", url);
    let key = "4938dO734UQ3NX9hZu7mGJMBRh5cqdWkNbeIhelfEO8xQXvF9GEdvfvfcaeAqc4v";
    let response = client.get(full_url)
        .header("Authorization", format!("Bearer {}", key))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::Error(format!("Failed to fetch {}: {}", url, response.status())));
    }

    Ok(response.text().await?)
}

pub async fn display_course(id : u32) -> Result<(), AppError> {
    let mut response = send_canvas_request(
        &format!("courses/{}", id))
        .await?;
    let course = serde_json::from_str::<Course>(&response)?;
    println!("Course ID: {}, Name: {}", course.id, course.name);

    response = send_canvas_request(
        &format!("courses/{}/assignment_groups?include[]=assignments&per_page=999",id))
        .await?;
    let assignment_groups = serde_json::from_str::<Vec<AssignmentGroup>>(&response)?;

    for assignment_group in assignment_groups.iter()  {
        println!("\n{:<50} {:<11} {:<7} {:<7}",format!("{} ({:.0}%)", assignment_group.name, assignment_group.group_weight.unwrap_or(0.0)),"Submissions","Average","ToGrade");
        println!("{} {} {} {}", "-".repeat(50), "-".repeat(11), "-".repeat(7), "-".repeat(7));
        if let Some(assignment) = &assignment_group.assignments {
            for assignment in assignment.iter() {
                let name = if assignment.group_category_id.is_some() {
                    format!("{} (Group)", assignment.name)
                } else {
                    assignment.name.clone()
                };
                // Submission Totals = add up all attempts for all current graded
                // Submission Grade = add up all scors for all current graded
                response = send_canvas_request(
                    &format!("courses/{}/assignments/{}/submissions?per_page=999", id, assignment.id))
                    .await?;
                let submissions_all = serde_json::from_str::<Vec<Submission>>(&response)?;        
                let submissions = submissions_all.into_iter()
                    .filter(|s| s.grade_matches_current_submission)
                    .collect::<Vec<Submission>>();
                let submission_count = submissions.iter()
                    .map(|s| s.attempt.unwrap_or(0))
                    .sum::<u32>();
                let student_count = submissions.iter()
                    .filter(|s| )
                let excused_count = submissions.iter()
                    .filter(|s| s.excused.unwrap_or(false))
                    .count();
                let count = course.total_students.unwrap_or(0) - excused_count as u32;
                let sum_scores = submissions.iter()
                    .map(|s| s.score.unwrap_or(0.0))
                    .sum::<f64>();
                let avg = if count > 0 {
                    sum_scores / count as f64
                } else {
                    0.0 
                };
                let avg_pct = if avg > 0.0 && assignment.points_possible.is_some() {
                    (avg / assignment.points_possible.unwrap()) * 100.0
                } else {
                    0.0
                };
                println!("{:<50} {:<11} {:<7} {:<7}",
                        name, submission_count, format!("{:.0} %",avg_pct), assignment.needs_grading_count.unwrap_or(0));    
            }
            
        }
    }    

    Ok(())


}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // display_course(351068).await?;
    display_course(335112).await?;
    Ok(())
}
