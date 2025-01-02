use rocket::State;
use rocket::serde::{Serialize, json::Json};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx;

use crate::config::Config;
use crate::database::DBPool;


#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(sqlx::FromRow)]
pub struct QueryCurrentGrades {
    name : String,
    submitted : i64,
    missing : i64,
    excused : i64,
    grade_a : i64,
    grade_b : i64,
    grade_c : i64,
    grade_d : i64,
    grade_f : i64,
    grade_zero : i64,
    avg_score : Option<f64>,
    avg_grade : Option<f64>,
    avg_grade_nonzero : Option<f64>,
    group : i32,
    ungraded_init : i64,
    ungraded_resubmit : i64
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CurrentGrades {
    count : usize,
    status : u32,
    message : String,
    data : Vec<QueryCurrentGrades>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(sqlx::FromRow)]
pub struct QueryCurrentStudents {
    name : String,
    submitted : i64,
    missing : i64,
    excused : i64,
    ungraded_init : i64,
    ungraded_resubmit : i64,
    curr_grade : String,
    curr_score : f32
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CurrentStudents {
    count : usize,
    status : u32,
    message : String,
    data : Vec<QueryCurrentStudents>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(sqlx::FromRow)]
struct QueryCurrentCourses {
    code : String,
    term : String,
    students : i32
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CurrentCourses {
    count : usize,
    status : u32,
    message : String,
    data : Vec<QueryCurrentCourses>,
}
// Routes

#[get("/current/grades/<course>")]
pub async fn route_current_grades(mut db: Connection<DBPool>, 
                              config : &State<Config>,
                              course : &str) -> Json<CurrentGrades> {
    let course_id = match config.current_courses.get(course) {
        Some(id) => *id,
        None => {
            return Json(CurrentGrades {data : vec![], count : 0, status : 400, message : format!("Invalid Course: {course}")});
        }
    };                             
    let query_data = sqlx::query_as::<_,QueryCurrentGrades>(
            "
            SELECT 
                asn.name,
                SUM(CASE WHEN sub.attempt > 0 THEN 1 ELSE 0 END) as submitted,
                SUM(CASE WHEN sub.missing THEN 1 ELSE 0 END) as missing,
                SUM(CASE WHEN sub.excused THEN 1 ELSE 0 END) as excused,
                SUM(CASE WHEN sub.score >= (0.9 * asn.points_possible) THEN 1 ELSE 0 END) as grade_a,
                SUM(CASE WHEN sub.score >= (0.8 * asn.points_possible) and 
                              sub.score < (0.9 * asn.points_possible) THEN 1 ELSE 0 END) as grade_b,
                SUM(CASE WHEN sub.score >= (0.7 * asn.points_possible) and 
                              sub.score < (0.8 * asn.points_possible) THEN 1 ELSE 0 END) as grade_c,
                SUM(CASE WHEN sub.score >= (0.6 * asn.points_possible) and 
                              sub.score < (0.7 * asn.points_possible) THEN 1 ELSE 0 END) as grade_d,
                SUM(CASE WHEN sub.score > 0 and 
                              sub.score < (0.6 * asn.points_possible) THEN 1 ELSE 0 END) as grade_f,
                SUM(CASE WHEN sub.score = 0 THEN 1 ELSE 0 END) as grade_zero,
                AVG(sub.score) as avg_score,
                CASE WHEN asn.points_possible = 0 THEN 0 
                     ELSE AVG(sub.score) / asn.points_possible * 100 END as avg_grade,
                CASE WHEN asn.points_possible = 0 THEN 0 
                     ELSE AVG(CASE WHEN sub.score > 0 THEN sub.score ELSE NULL END) / asn.points_possible * 100 END as avg_grade_nonzero,
                asn.assignment_group_id as group,
                SUM(CASE WHEN sub.score IS NULL and sub.attempt > 0 THEN 1 ELSE 0 END) as ungraded_init,
                SUM(CASE WHEN not sub.current_submission THEN 1 ELSE 0 END) as ungraded_resubmit
            FROM curr_assignments AS asn
            INNER JOIN curr_submissions AS sub
                ON asn.id = sub.assignment_id
            INNER JOIN curr_students as stu
                ON stu.id = sub.user_id
            WHERE asn.course_id = $1 and stu.course_id = $1 and asn.points_possible > 0
            GROUP BY asn.name, asn.points_possible, asn.assignment_group_id
            ORDER BY asn.assignment_group_id, asn.name;
        ")
        .bind(course_id)
        .fetch_all(&mut **db)
        .await;
    match query_data {
        Ok(data) => Json(CurrentGrades {count : data.len(), data, status : 200, message : "OK".to_string()}),
        Err(error) => Json(CurrentGrades {count : 0, data : vec![], status : 500, message : format!("SQL Error: {error}")})
    }
}

#[get("/current/students/<course>")]
pub async fn route_current_students(mut db: Connection<DBPool>, 
                                config : &State<Config>,
                                course : &str) -> Json<CurrentStudents> {

    let course_id = match config.current_courses.get(course) {
        Some(id) => *id,
        None => {
            return Json(CurrentStudents {data : vec![], count : 0, status : 400, message : format!("Invalid Course: {course}")});
        }
    };    
    let query_data = sqlx::query_as::<_,QueryCurrentStudents>(
            "
            SELECT 
                stu.name,
                SUM(CASE WHEN sub.attempt > 0 THEN 1 ELSE 0 END) as submitted,
                SUM(CASE WHEN sub.missing THEN 1 ELSE 0 END) as missing,
                SUM(CASE WHEN sub.excused THEN 1 ELSE 0 END) as excused,
                SUM(CASE WHEN sub.score IS NULL and sub.attempt > 0 THEN 1 ELSE 0 END) as ungraded_init,
                SUM(CASE WHEN not sub.current_submission THEN 1 ELSE 0 END) as ungraded_resubmit,
                stu.curr_grade,
                stu.curr_score
            FROM curr_students AS stu
            INNER JOIN curr_submissions AS sub
                ON stu.id = sub.user_id
            INNER JOIN curr_assignments as asn
                ON asn.id = sub.assignment_id
            WHERE asn.course_id = $1 and stu.course_id = $1 and asn.points_possible > 0
            GROUP BY stu.name, stu.curr_grade, stu.curr_score
            ORDER BY stu.curr_score DESC, stu.name ASC;
        ")
        .bind(course_id)
        .fetch_all(&mut **db)
        .await;
    match query_data {
        Ok(data) => Json(CurrentStudents {count : data.len(), data, status : 200, message : "OK".to_string()}),
        Err(error) => Json(CurrentStudents {count : 0, data : vec![], status : 500, message : format!("SQL Error: {error}")})
    }
}

#[get("/current/courses")]
pub async fn route_current_courses(mut db: Connection<DBPool>) -> Json<CurrentCourses> {
    let query_data = sqlx::query_as::<_,QueryCurrentCourses>(
            "
            SELECT code, term, students
            FROM curr_courses
            ORDER BY code;
        ")
        .fetch_all(&mut **db)
        .await;
    match query_data {
        Ok(data) => Json(CurrentCourses {count : data.len(), data, status : 200, message : "OK".to_string()}),
        Err(error) => Json(CurrentCourses {count : 0, data : vec![], status : 500, message : format!("SQL Error: {error}")})
    }
}
