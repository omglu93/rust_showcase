use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use std::collections::HashMap;
use dotenv::dotenv;

use std::env;

/// Figment application configuration ///

pub struct ApplicationState {
    pub secret: Vec<u8>
}

impl ApplicationState {

    pub fn manage() -> AdHoc {

        dotenv().ok();
        AdHoc::on_ignite("Manage cofig", |rocket| async move {
            let secret_key = env::var("SECRET_KEY").unwrap_or_else(|err| {
                panic!("No SECRET_KEY environment variable found: {:?}", err)
            });

            rocket.manage(ApplicationState {
                secret: secret_key.into_bytes()
            })
        })
    }
}

pub fn figment() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");


    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    
    let database_url = env::var("DATABASE_URL").expect("No database url found in env");

    database_config.insert("url", database_url);
    databases.insert("diesel_postgres_pool", database_config);

    Config::figment()
        .merge(("port", port))
        .merge(("databases", databases))
}
