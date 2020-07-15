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

use daemonize::Daemonize;
use std::io::prelude::*;
use structopt::StructOpt;

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

async fn start_server() -> Result<()> {
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
                actix_files::Files::new("/app", "./webapp/dist/")
                    .show_files_listing()
                    .index_file("index.html"),
            )
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

fn init_config() -> Result<()> {
    Ok(())
}

fn init_logging(opt: &Opt) -> Result<()> {
    let log_config = simplelog::ConfigBuilder::new().build();

    let log_level = match opt {
        Opt::Run { debug, .. } | Opt::Start { debug, .. } => {
            if *debug {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            }
        }
        _ => LevelFilter::Info,
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

    match opt {
        Opt::Start { log_file, .. } => {
            if let Some(log_file) = log_file {
                loggers.push(simplelog::WriteLogger::new(
                    log_level,
                    log_config.clone(),
                    File::create(log_file)?,
                ));
            }
        }
        _ => {}
    }

    // Combine all these loggers
    simplelog::CombinedLogger::init(loggers).expect("Could not initialize logging");

    info!("Logging with level {}", log_level);
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "roompla",
    about = "Roompla office room planner service and web application."
)]

enum Opt {
    Run {
        #[structopt(short, long, help = "Output debug messages in log")]
        debug: bool,
    },
    Start {
        #[structopt(short, long, help = "Output debug messages in log")]
        debug: bool,
        #[structopt(short, long, help = "The location of the output log file")]
        log_file: Option<String>,
    },
    Stop {},
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Init configuration and logging
    init_config()?;
    init_logging(&opt)?;

    match opt {
        Opt::Run { .. } => {
            // Directly run server
            start_server().await
        }
        Opt::Start { .. } => {
            // Start server in and daemonize this process
            let daemonize = Daemonize::new()
                .pid_file("/tmp/roompla.pid")
                .working_directory(".");

            daemonize
                .start()
                .expect("Could not start service in background");

            start_server().await?;
            println!("Yeah, server started");
            Ok(())
        }
        Opt::Stop { .. } => {
            let mut pid_raw = String::new();
            File::open("/tmp/roompla.pid")?.read_to_string(&mut pid_raw)?;
            if let Ok(pid) = pid_raw.parse::<heim::process::Pid>() {
                // Check if this process exists and terminate it
                if let Ok(process) = heim::process::get(pid).await {
                    match process.terminate().await {
                        Ok(_) => info!("Process {} terminated", pid),
                        Err(e) => error!("Could not terminate process {}: {}", pid, e),
                    }
                } else {
                    warn!("Process {} not found", pid);
                }
            }
            Ok(())
        }
    }
}
