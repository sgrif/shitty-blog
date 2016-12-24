#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel_rocket;
extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::Template;
use diesel_rocket::Post;

#[get("/")]
fn index() -> Template {
    let posts = vec![
        Post::new(1, "First Post", "This is my first post"),
        Post::new(2, "Second Post", "This is my second post"),
    ];
    let context = map!["posts" => posts];
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
