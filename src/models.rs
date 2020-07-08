use super::schema::*;
use chrono::prelude::*;

#[derive(Queryable, Insertable, Serialize)]
pub struct Room {
    pub id: String,
    pub max_occupancy: i32,
}

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub password_hash: Option<String>,
}

#[derive(Insertable, Serialize, Clone)]
#[table_name = "occupancies"]
pub struct NewOccupancy {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user: String,
    pub room: String,
}

#[derive(QueryableByName, Queryable)]
#[table_name = "occupancies"]
pub struct Occupancy {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user: String,
    pub room: String,
}
