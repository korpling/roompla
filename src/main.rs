#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use simplelog::{LevelFilter, SimpleLogger, TermLogger};
use std::{
    env,
    io::{Error, ErrorKind, Result},
};

use actix_web::{
    http::{self, ContentEncoding},
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpServer,
};

pub mod api;
pub mod errors;
pub mod extractors;
pub mod models;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn open_db_pool() -> anyhow::Result<DbPool> {
    dotenv()?;

    let database_url = env::var("DATABASE_URL")?;

    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let db_pool = r2d2::Pool::builder().build(manager)?;
    Ok(db_pool)
}

async fn get_api_spec(_req: HttpRequest) -> web::HttpResponse {
    web::HttpResponse::Ok()
        .content_type("application/x-yaml")
        .body(include_str!("openapi.yml"))
}

#[actix_rt::main]
async fn main() -> Result<()> {
    // Init logging
    let log_filter = LevelFilter::Debug;

    let log_config = simplelog::ConfigBuilder::new()
        .add_filter_ignore_str("rustyline:")
        .build();

    if let Err(e) = TermLogger::init(
        log_filter,
        log_config.clone(),
        simplelog::TerminalMode::Mixed,
    ) {
        println!("Error, can't initialize the terminal log output: {}.\nWill degrade to a more simple logger", e);
        if let Err(e_simple) = SimpleLogger::init(log_filter, log_config) {
            println!("Simple logging failed too: {}", e_simple);
        }
    }

    info!("Logging with level {}", log_filter);

    let db_pool = open_db_pool().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Could not initialize service: {:?}", e),
        )
    })?;

    let bind_address = format!("localhost:5050");
    let api_version = format!("/v{}", env!("CARGO_PKG_VERSION_MAJOR"),);

    let db_pool = web::Data::new(db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .wrap(
                Cors::new()
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(Compress::new(ContentEncoding::Gzip))
            .service(
                web::scope(&api_version)
                    .route("openapi.yml", web::get().to(get_api_spec))
                    .route("/login", web::post().to(api::login))
                    .route(
                        "/rooms/{room}/occupancies",
                        web::put().to(api::add_occupancy),
                    )
                    .route(
                        "/rooms/{room}/occupancies",
                        web::get().to(api::get_occupancies),
                    )
                    .route("/rooms", web::get().to(api::all_rooms)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
