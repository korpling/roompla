use super::schema::*;
use chrono::prelude::*;

#[derive(Queryable, Insertable)]
pub struct Room {
    pub id: String,
    pub max_occupancy: i32,
}

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub password_hash: Option<String>,
}

#[derive(Insertable)]
#[table_name="presencies"]
pub struct NewPresency {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub user: String,
    pub room: String,
}


#[derive(QueryableByName)]
#[table_name="presencies"]
pub struct Presency {
    pub id: i32,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub user: String,
    pub room: String,
}