extern crate web_api;

use web_api::config::Config;
use web_api::database::DBPool;
use web_api::{routes_current, routes_trends};
use rocket::routes;
use rocket_db_pools::Database;


#[rocket::main]
async fn main() {
    let config = match Config::load_config() {
        Ok(result) => result,
        Err(error) => {
            println!("{}", error);
            println!("Goodbye!");    
            return;
        }
    };

    println!("Config: {:?}", config);

    let launch_result = rocket::build() 
        .attach(DBPool::init())
        .manage(config)
        .mount("/", routes![routes_current::route_current_grades, 
                            routes_current::route_current_courses, 
                            routes_current::route_current_students,
                            routes_trends::route_trends_trend])
        .launch()
        .await;
    if let Err(error) = launch_result {
        println!("{}", error);
    }
    println!("Goodbye!");
}


// Add routes for update_current and upddate_trends and get courses for trends
// Add authentication
// Create app to call update every hour
// Create React Front End
