#[macro_use]
extern crate diesel;
extern crate dotenv;

use anyhow::Error;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

fn open_db_pool() -> Result<DbPool, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
    let db_pool = r2d2::Pool::builder().build(manager)?;
    Ok(db_pool)
}

fn main() -> Result<(), Error> {
    let db_pool = open_db_pool()?;

    // Insert some test data
    let conn = db_pool.get()?;

    let new_room = models::Room {
        id: "3.333".to_string(),
        max_occupancy: 2,
    };
    let new_user = models::User {
        id: "krauseto".to_string(),
        password_hash: None,
    };
    let now = Utc::now();
    let new_event = models::NewPresency {
        from: now.naive_local(),
        to: (now + Duration::hours(1)).naive_local(),
        user: "krauseto".to_string(),
        room: "3.333".to_string(),
    };

    diesel::insert_into(schema::rooms::table)
        .values(new_room)
        .execute(&conn)?;

    diesel::insert_into(schema::users::table)
        .values(new_user)
        .execute(&conn)?;

    diesel::insert_into(schema::presencies::table)
        .values(new_event)
        .execute(&conn)?;

    Ok(())
}
