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
use daemonize::Daemonize;
use heim::process::Pid;
use std::io::prelude::*;
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

    let bind_address = format!("localhost:{:?}", &settings.service);
    let api_version = format!("/v{}", env!("CARGO_PKG_VERSION_MAJOR"),);

    let db_pool = web::Data::new(db_pool);

    let settings = web::Data::new(settings);

    HttpServer::new(move || {
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

fn init_config(opt: &Opt) -> anyhow::Result<Settings> {
    let mut settings = match opt {
        Opt::Run { config, .. } | Opt::Start { config, .. } | Opt::Restart { config, .. } => {
            Settings::with_file(config)?
        }
        _ => Settings::new()?,
    };

    // Initialize with some default
    if settings.jwt.secret.is_none() {
        settings.jwt.secret = Some(thread_rng().sample_iter(&Alphanumeric).take(30).collect());
    }
    Ok(settings)
}

fn init_logging(opt: &Opt, settings: &Settings) -> Result<()> {
    let log_config = simplelog::ConfigBuilder::new().build();

    let log_level = match opt {
        Opt::Run { debug, .. } | Opt::Start { debug, .. } | Opt::Restart { debug, .. } => {
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

#[derive(Debug, StructOpt)]
#[structopt(
    name = "roompla",
    about = "Roompla office room planner service and web application."
)]

enum Opt {
    #[structopt(help = "Run the service in the foreground")]
    Run {
        #[structopt(short, long, help = "Output debug messages in log")]
        debug: bool,
        #[structopt(short, long, help = "Configuration file")]
        config: Option<String>,
    },
    #[structopt(help = "Start as background service")]
    Start {
        #[structopt(short, long, help = "Output debug messages in log")]
        debug: bool,
        #[structopt(short, long, help = "Configuration file")]
        config: Option<String>,
    },
    #[structopt(help = "Restart background service")]
    Restart {
        #[structopt(short, long, help = "Output debug messages in log")]
        debug: bool,
        #[structopt(short, long, help = "Configuration file")]
        config: Option<String>,
    },
    #[structopt(help = "Stop running background service")]
    Stop {},
    Export {
        #[structopt(help = "The output CSV file")]
        file: String,
        #[structopt(short, long, help = "How many weeks to include", default_value = "2")]
        weeks: u8,
    },
}

async fn check_running(settings: &Settings) -> Option<Pid> {
    let mut pid_raw = String::new();
    if let Ok(mut f) = File::open(&settings.service.pidfile) {
        if let Ok(_) = f.read_to_string(&mut pid_raw) {
            if let Ok(pid) = pid_raw.parse::<Pid>() {
                if let Ok(_) = heim::process::get(pid).await {
                    return Some(pid);
                }
            }
        }
    }
    None
}

async fn start(settings: Settings) -> Result<()> {
    if let Some(pid) = check_running(&settings).await {
        error!("Service is already running with PID {}", pid);
    } else {
        info!("Starting service");
        // Start server in and daemonize this process
        let mut daemonize = Daemonize::new()
            .pid_file(&settings.service.pidfile);
        if let Some(user) = &settings.service.user {
            daemonize = daemonize.user(user.as_ref());
        }
        if let Some(group) = &settings.service.group {
            daemonize = daemonize.group(group.as_ref());
        }
        if let Some(working_directory) = &settings.service.path {
            daemonize = daemonize.working_directory(working_directory);
        } else {
            daemonize = daemonize.working_directory(".");
        }

        daemonize
            .start()
            .expect("Could not start service in background");

        run_server(settings).await?;
    }
    Ok(())
}

async fn stop(settings: Settings) -> Result<()> {
    let mut pid_raw = String::new();
    File::open(&settings.service.pidfile)?.read_to_string(&mut pid_raw)?;
    if let Ok(pid) = pid_raw.parse::<heim::process::Pid>() {
        // Check if this process exists and terminate it
        if let Ok(process) = heim::process::get(pid).await {
            match process.terminate().await {
                Ok(_) => {
                    info!("Process {} terminated", pid);
                    // delete the PID file
                    std::fs::remove_file(&settings.service.pidfile)?;
                }
                Err(e) => error!("Could not terminate process {}: {}", pid, e),
            }
        } else {
            warn!("Process {} not found", pid);
        }
    } else {
        warn!("No PID file found at {}", &settings.service.pidfile);
    }
    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Init configuration and logging
    let settings = init_config(&opt).map_err(|e| {
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
    init_logging(&opt, &settings)?;

    match opt {
        Opt::Run { .. } => {
            // Directly run server
            run_server(settings).await
        }
        Opt::Start { .. } => start(settings).await,
        Opt::Stop { .. } => stop(settings).await,
        Opt::Restart { .. } => {
            stop(settings.clone()).await?;
            start(settings).await
        }
        Opt::Export { file, weeks } => {
            match export::to_csv(&file, weeks, settings) {
                Ok(result) => futures::future::ok(result),
                Err(e) => {
                    error!("Error when exporting to CSV: {:?}", e);
                    futures::future::err(std::io::Error::new(ErrorKind::Other, format!("{:?}", e)))
                }
            }
            .await
        }
    }
}
