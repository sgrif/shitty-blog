#![feature(proc_macro)]

#[macro_use] extern crate serde_derive;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Post {
    id: i64,
    title: String,
    body: String,
}
