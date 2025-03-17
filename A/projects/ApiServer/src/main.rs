use api_server::rocket;

// #[launch]
// fn rocket_main() -> _ {
//     rocket()
// }


#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};
use  rocket::response::status;
use rocket_sync_db_pools::database;
use diesel::QueryDsl;


use self::models::*;
use self::schema::rustaceans::dsl::*;
use diesel::prelude::*;
use diesel::ExpressionMethods;



mod auth;
mod schema;
mod models;
use auth::BasicAuth;
use schema::rustaceans;
use models::Rustacean;

//database attribute comes from rocket_sync_db_pools where we pass the url of the database
//The url is the path to the database file
//The struct DbConn is a wrapper around the diesel::SqliteConnection
//The diesel::SqliteConnection is the connection to the database
//The DbConn struct is used to pass the connection to the database to the handlers
//The handlers are the functions that handle the requests
#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/he")]
fn hello() -> Value {
    json!(["Hello, world!", "Hello, world!"])
}
#[get("/rustaceans")]
async fn get_rustaceans(_db: DbConn) -> Value {

    // _db.run(|c| { 
    //     let rustaceans 
    //     = rustaceans::table.order(rustaceans::id.desc())
    //     .limit(1000).load::<Rustacean>(c)
    //     .expect("DB Error loading rustaceans");
    //     json!(rustaceans)
    // }).await 
    json!([{ "id":1, "name":"Alice", "email":""}])
}
#[get("/rustaceans/<id>")]
fn view_rustaceans(_db: DbConn) -> Value {
    json!([{ "id":1, "name":"Alice", "email":"john doe"}])
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_db: DbConn, _auth: BasicAuth) -> Value {
    json!([{ "id":1, "name":"Alice", "email":"john doe"}])
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(_db: DbConn) -> Value {
    json!([{ "id":1, "name":"Alice", "email":"john doe"}])
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(_id: i32, _db: DbConn) -> status::NoContent {
    status::NoContent
    
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Invalid/Missing authorization")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
    .mount("/", routes![hello, get_rustaceans, view_rustaceans, create_rustacean, update_rustaceans, delete_rustacean]) 
    .register("/", catchers![
        not_found,
        unauthorized
        ])
    .attach(DbConn::fairing())
    .launch().await;
}
