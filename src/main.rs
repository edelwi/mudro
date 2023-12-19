extern crate dotenv;
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::{error, info};

mod crud {
    pub mod author;
    pub mod quote;
}

mod models {
    pub mod author;
    pub mod parameter;
    pub mod quote;
}

mod routs {
    pub mod authors;
    pub mod main;
    pub mod quotes;
}
use routs::main::config;

use dotenv::dotenv;
use std::env;

pub struct AppState {
    db: sqlx::postgres::PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Mudro started.");
    let db_url = "DATABASE_URL";
    let url = env::var(db_url).expect(&format!("Expected {} to be set", db_url));
    let bind_addr = "HOST";
    let host = env::var(bind_addr).expect(&format!("Expected {} to be set", bind_addr));
    let bind_port = "PORT";
    let port: u16 = env::var(bind_port)
        .expect(&format!("Expected {} to be set", bind_port))
        .parse()
        .expect("PORT must be a valid port number");

    let pool = sqlx::postgres::PgPool::connect(url.as_str());

    let pool = match sqlx::postgres::PgPool::connect(url.as_str()).await {
        Ok(pool) => {
            info!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    info!("running db migrations");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => (),
        Err(e) => {
            error!("Error running migrations: {:?}", e);
            std::process::exit(2);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(config)
            .wrap(Logger::default())
    })
    .bind((host, port))?
    .run()
    .await
}
