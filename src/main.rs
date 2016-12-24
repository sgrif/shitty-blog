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
use rocket::response::Redirect;
use rocket::request::Form;

#[get("/")]
fn index() -> Template {
    let connection = connection();
    let posts = posts::table.load::<Post>(&connection)
        .expect("Failed to load posts");
    let context = map!["posts" => posts];
    Template::render("index", &context)
}

#[get("/posts/new")]
fn new_post() -> Template {
    let context = map!["title" => "New Post"];
    Template::render("new_post", &context)
}

#[post("/posts", data = "<post>")]
fn create_post(post: Form<NewPost>) -> Redirect {
    let connection = connection();
    diesel::insert(post.get())
        .into(posts::table)
        .execute(&connection)
        .expect("Failed to create post");
    Redirect::found("/")
}

#[delete("/posts/<id>")]
fn delete_post(id: i64) -> Redirect {
    let connection = connection();
    diesel::delete(posts::table.find(id))
        .execute(&connection)
        .expect("Failed to delete post");
    Redirect::found("/")
}

fn main() {
    rocket::ignite().mount("/", routes![index, new_post, create_post, delete_post]).launch();
}
