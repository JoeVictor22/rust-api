#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Neuromancer"),
        author: String::from("William Gibson")m
        published: true,
    }

    if models:Book::insert(book, &conn) {
        println!("Show");
    } else {
        println!("deu ruim boy");
    }
}
