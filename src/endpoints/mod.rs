pub mod calculation;
pub mod io_operation;
pub mod database;

use rocket_sync_db_pools::database;

#[database("symptalk_db")]
pub struct AppDatabase(diesel::PgConnection);
