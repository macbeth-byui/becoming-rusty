use rocket_db_pools::Database;
use rocket_db_pools::sqlx;

#[derive(Database)]
#[database("horizons")]
pub struct DBPool(sqlx::PgPool);

// See Rocket.toml