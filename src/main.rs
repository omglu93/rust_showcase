#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod endpoints;
mod config;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api/v1/", routes![endpoints::calculation::post_cpu_operation,
    endpoints::io_operation::get_io_operation,
    endpoints::database::post_database_operation])
    .attach(config::ApplicationState::manage())
    .attach(endpoints::AppDatabase::fairing())
}
