
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};

use super::schema::rustaceans;  // Important: Bring in the table definition



#[derive(Serialize, Queryable)]
#[diesel(check_for_backend(Sqlite))]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime
}
#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String
}

