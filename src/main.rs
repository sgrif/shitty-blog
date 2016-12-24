#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel_rocket;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let context = map!["title" => None::<String>];
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
