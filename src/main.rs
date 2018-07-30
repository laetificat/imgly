#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(extern_prelude)]

extern crate chrono;

extern crate dotenv;

#[macro_use] extern crate serde_derive;

extern crate serde_json;
extern crate serde;

extern crate uuid;

extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate diesel;

mod routing;
mod entity;
mod schema;
mod helper;

use routing::{ root, user, album, picture };

use diesel::pg::PgConnection;
use diesel::Connection;

use dotenv::dotenv;

use std::env;

fn main() {
    rocket::ignite()
        .mount("/", routes![root::index])
        .mount("/users", routes![user::list, user::get_user, user::create])
        .mount("/users", routes![album::list, album::get_album, album::create])
        .mount("/users", routes![picture::list, picture::upload])
        .launch();
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}