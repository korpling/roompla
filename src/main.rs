#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use actix_cors::Cors;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use simplelog::{LevelFilter, SharedLogger, SimpleLogger, TermLogger};
use std::{
    env,
    fs::File,
    io::{Error, ErrorKind, Result},
};

use actix_web::{
    http::{self, ContentEncoding},
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpServer,
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::config::Settings;
use dotenv::dotenv;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod api;
pub mod config;
pub mod errors;
pub mod export;
pub mod extractors;
pub mod models;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

embed_migrations!("migrations");
use std::collections::HashMap;
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn open_db_pool(settings: &Settings) -> anyhow::Result<DbPool> {
    info!("Loading database from {}", &settings.database.url);
    let manager = ConnectionManager::<SqliteConnection>::new(&settings.database.url);
    let db_pool = r2d2::Pool::builder().build(manager)?;

    // Make sure the database has all migrations applied
    let conn = db_pool.get()?;
    embedded_migrations::run(&conn)?;

    Ok(db_pool)
}

async fn get_api_spec(_req: HttpRequest) -> web::HttpResponse {
    web::HttpResponse::Ok()
        .content_type("application/x-yaml")
        .body(include_str!("openapi.yml"))
}

async fn run_server(settings: Settings) -> Result<()> {
    let db_pool = open_db_pool(&settings).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("Could not initialize service: {:?}", e),
        )
    })?;

    let bind_address = format!("localhost:{}", &settings.service.port);
    let api_version = format!("/roompla/v{}", env!("CARGO_PKG_VERSION_MAJOR"),);

    let db_pool = web::Data::new(db_pool);

    let settings = web::Data::new(settings);

    HttpServer::new(move || {
        let generated = generate();

        App::new()
            .app_data(db_pool.clone())
            .app_data(settings.clone())
            .wrap(
                Cors::new()
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .finish(),
            )
            .wrap(Logger::default())
            .wrap(Compress::new(ContentEncoding::Gzip))
            .service(actix_web_static_files::ResourceFiles::new(
                "/roompla/app",
                generated,
            ))
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
                    .route(
                        "/rooms/{room}/occupancies/{id}",
                        web::put().to(api::update_occupancy),
                    )
                    .route(
                        "/rooms/{room}/occupancies/{id}",
                        web::delete().to(api::delete_occupancy),
                    )
                    .route("/rooms", web::get().to(api::all_rooms)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}

fn init_config() -> anyhow::Result<(PathBuf, Settings)> {
    dotenv().ok();

    // Check if a configuration file is given via the environment
    let config_file = env::var("ROOMPLA_CONFIG")
        .ok()
        .unwrap_or_else(|| "roompla.toml".to_string());

    let config_file = PathBuf::from(config_file);

    let mut settings = if config_file.is_file() {
        Settings::with_file(config_file.to_string_lossy())?
    } else {
        Settings::new()?
    };

    // Initialize with some default
    if settings.jwt.secret.is_none() {
        settings.jwt.secret = Some(thread_rng().sample_iter(&Alphanumeric).take(30).collect());
    }

    Ok((config_file, settings))
}

fn init_logging(settings: &Settings) -> Result<()> {
    let log_config = simplelog::ConfigBuilder::new().build();

    let log_level = if settings.log.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::default();
    if let Some(term_logger) = TermLogger::new(
        log_level,
        log_config.clone(),
        simplelog::TerminalMode::Mixed,
    ) {
        loggers.push(term_logger);
    } else {
        // Use a more simple terminal logger
        loggers.push(SimpleLogger::new(log_level, log_config.clone()));
    }

    if let Some(logfile) = &settings.service.logfile {
        loggers.push(simplelog::WriteLogger::new(
            log_level,
            log_config.clone(),
            File::create(logfile)?,
        ));
    }

    // Combine all these loggers
    simplelog::CombinedLogger::init(loggers).expect("Could not initialize logging");

    info!("Logging with level {}", log_level);
    Ok(())
}

#[derive(StructOpt)]
enum Command {
    Export {
        #[structopt(help = "The output CSV file")]
        file: String,
        #[structopt(short, long, help = "How many weeks to include", default_value = "2")]
        weeks: u8,
    },
}

#[derive(StructOpt)]
#[structopt(
    name = "roompla",
    about = "Roompla office room planner service and web application."
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Init configuration and logging
    let (config_file_location, settings) = init_config().map_err(|e| {
        let cause: Vec<String> = e.chain().skip(1).map(|c| format!("{}", c)).collect();
        Error::new(
            ErrorKind::Other,
            format!(
                "Could not load configuration: {:?}\n{}",
                e,
                cause.join("\n")
            ),
        )
    })?;
    init_logging(&settings)?;

    info!(
        "Attempting to load configuration from {}",
        config_file_location.to_string_lossy()
    );

    if let Some(cmd) = opt.cmd {
        match cmd {
            Command::Export { file, weeks } => {
                match export::to_csv(&file, weeks, settings) {
                    Ok(result) => futures::future::ok(result),
                    Err(e) => {
                        error!("Error when exporting to CSV: {:?}", e);
                        futures::future::err(std::io::Error::new(
                            ErrorKind::Other,
                            format!("{:?}", e),
                        ))
                    }
                }
                .await
            }
        }
    } else {
        // Directly run server
        run_server(settings).await
    }
}
