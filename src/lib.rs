#![feature(proc_macro)]

#[macro_use] extern crate serde_derive;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Post {
    id: i64,
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
