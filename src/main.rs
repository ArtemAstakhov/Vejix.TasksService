#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_cors;
extern crate tokio;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, Error};

mod db;
mod user_inject;
mod schema;
mod models;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello from vejix user service!"
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_methods: vec![
          Method::Get,
          Method::Post,
          Method::Delete,
          Method::Put
        ].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
      .attach(cors)
      .manage(db::connect())
      .mount("/", routes![index])
      .mount("/api/v1/tasks", routes![
        routes::tasks::read,
        routes::tasks::create,
        routes::tasks::delete,
        routes::tasks::reorder,
        routes::tasks::update
      ])
      .launch();

    Ok(())
}
