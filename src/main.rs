#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel_rocket;
extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;

use diesel::prelude::*;
use diesel_rocket::*;
use diesel_rocket::schema::posts;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let connection = connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    let context = map!["posts" => posts];
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
