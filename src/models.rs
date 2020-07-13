use super::schema::*;
use chrono::prelude::*;

#[derive(Queryable, Insertable, Serialize)]
pub struct Room {
    pub id: String,
    pub max_occupancy: i32,
    pub timezone: Option<String>,
}

#[derive(Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub display_name: String,
    pub contact_info: String,
    pub password_hash: Option<String>,
}

#[derive(Insertable, Serialize, Clone)]
#[table_name = "occupancies"]
pub struct NewOccupancy {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: String,
    pub user_name: String,
    pub user_contact: String,
    pub room: String,
}

#[derive(QueryableByName, Queryable, Serialize, Debug)]
#[table_name = "occupancies"]
pub struct Occupancy {
    pub id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub user_id: String,
    pub user_name: String,
    pub user_contact: String,
    pub room: String,
}