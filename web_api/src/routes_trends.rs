use std::collections::HashMap;
use rocket::State;
use rocket::serde::{Serialize, json::Json};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;

use crate::config::Config;
use crate::database::DBPool;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(sqlx::FromRow)]
pub struct QueryTrendsTrend {
    name : String,
    grade : Option<f64>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TrendsTrend {
    count : usize,
    status : u32,
    message : String,
    data : HashMap<String, Vec<QueryTrendsTrend>>,
}

#[get("/trends/trend/<course>")]
pub async fn route_trends_trend(mut db: Connection<DBPool>, 
                              config : &State<Config>,
                              course : &str) -> Json<TrendsTrend> {
    let sections = match config.trends_config.get(course) {
        Some(sections) => {
            let mut section_ids = Vec::new();
            for section in sections {
                section_ids.push(section.0);
            }
            section_ids
        }
        None => {
            return Json(TrendsTrend {data : HashMap::new(), count : 0, status : 400, message : format!("Invalid Course: {course}")});
        }
    };                             

    #[derive(sqlx::FromRow, Debug)]
    struct TermQuery {
        term : String
    }

    let query_terms = sqlx::query_as::<_,TermQuery>(
        "SELECT DISTINCT course.term as term
                FROM trend_assignments AS asn
                INNER JOIN trend_submissions AS sub
                    ON asn.id = sub.assignment_id
                INNER JOIN trend_students as stu
                    ON stu.id = sub.user_id
                INNER JOIN trend_courses as course
                    ON course.id = asn.course_id
                WHERE asn.course_id = ANY($1) and stu.course_id = ANY($1) and asn.points_possible > 0
                ORDER BY course.term
        ")        
        .bind(&sections)
        .fetch_all(&mut **db)
        .await;

    let terms = match query_terms {
        Ok(terms) => terms,
        Err(error) => {
            return Json(TrendsTrend {data : HashMap::new(), count : 0, status : 400, message : format!("SQL Error (terms): {error}")});
        }
    };      

    let mut sql = "SELECT name".to_string();
    for term in terms.iter() {
        sql += format!(", MAX(CASE WHEN term = '{}' THEN avg_grade ELSE null END) AS avg_grade_{} ",
            term.term, term.term.replace('-',"_").to_lowercase()).as_str();
    }  
    
    sql += "FROM (
                SELECT 
                    asn.name as name,
                    course.term as term,
                    AVG(sub.score) / asn.points_possible * 100 as avg_grade
                FROM trend_assignments AS asn
                INNER JOIN trend_submissions AS sub
                    ON asn.id = sub.assignment_id
                INNER JOIN trend_students AS stu
                    ON stu.id = sub.user_id
                INNER JOIN trend_courses AS course
                    ON course.id = asn.course_id ";
    // if zero {
    sql += "WHERE asn.course_id = ANY($1) and stu.course_id = ANY($1) and asn.points_possible > 0 ";
    // }
    // else {
    //     sql += "WHERE asn.course_id = ANY($1) and stu.course_id = ANY($1) and asn.points_possible > 0 and sub.score > 0 ";
    // }
    sql += "GROUP BY asn.name, asn.points_possible, course.term
            )
            GROUP BY name
            ORDER BY name";
    let query_results = sqlx::query(&sql)
        .bind(&sections)
        .fetch_all(&mut **db)
        .await;

    let results = match query_results {
        Ok(results) => results,
        Err(error) => {
            return Json(TrendsTrend {data : HashMap::new(), count : 0, status : 400, message : format!("SQL Error (trends): {error}")});
        }
    };      

    let mut data = HashMap::new();
    for term in terms.iter() {
        let key = format!("avg_grade_{}", term.term.replace('-',"_").to_lowercase());
        let mut list = Vec::new();
        for result in results.iter() {
            let grade: Option<f64> = result.get(key.as_str());
            let name: String = result.get("name");
            list.push(QueryTrendsTrend {name, grade});
        }
        data.insert(key, list);
    }

    Json(TrendsTrend {count : data.len(), data, status : 200, message : "OK".to_string()})

}

