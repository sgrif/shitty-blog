#![feature(proc_macro, plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate dotenv;
extern crate rocket;

pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

use self::schema::posts;

#[derive(Debug, PartialEq, Eq, Serialize, Queryable)]
pub struct Post {
    id: i64,
    title: String,
    body: String,
}

#[derive(Debug, Insertable, FromForm, AsChangeset)]
#[table_name="posts"]
pub struct PostForm {
    title: String,
    body: String,
}

impl Post {
    pub fn new(id: i64, title: &str, body: &str) -> Self {
        Post {
            id: id,
            title: title.into(),
            body: body.into(),
        }
    }
}

pub fn connection() -> PgConnection {
    let _ = dotenv::dotenv();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set or be present in a .env file");
    PgConnection::establish(&database_url)
        .expect("Failed to establish database connection")
}
