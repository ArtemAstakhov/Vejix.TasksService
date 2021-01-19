#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod db;
mod user_inject;
mod schema;
mod models;
mod routes;
mod dto;

#[get("/")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("KAIZEN: welcome to tasks service!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  db::init();

  HttpServer::new(|| {
    let cors = Cors::default()
      .allow_any_origin()
      .allow_any_method()
      .allow_any_header()
      .send_wildcard();

      App::new()
        .wrap(cors)
        .app_data(actix_web::web::JsonConfig::default().error_handler(|err, _req| {
          actix_web::error::InternalError::from_response(
            "",
            HttpResponse::BadRequest()
              .content_type("application/json")
              .body(format!(r#"{{"error":"{}"}}"#, err)),
          )
          .into()
        }))
        .data(db::connection())
        .service(index)
        .service(
          web::scope("/api/v1")
            .service(
              web::scope("/tasks")
                .service(routes::tasks::read)
                .service(routes::tasks::create)
                .service(routes::tasks::delete)
                .service(routes::tasks::reorder)
                .service(routes::tasks::update)
                .service(routes::tasks::complete)
                .service(routes::tasks::uncomplete)
            )
            .service(
              web::scope("/tasks-lists")
                .service(routes::tasks_lists::read)
                .service(routes::tasks_lists::create)
                .service(routes::tasks_lists::delete)
                .service(routes::tasks_lists::update)
            )
            .service(
              web::scope("/checklists")
                .service(routes::checklists::read)
                .service(routes::checklists::create)
                .service(routes::checklists::delete)
                .service(routes::checklists::update)
            )
            .service(
              web::scope("/checklist-items")
                .service(routes::checklist_items::read)
                .service(routes::checklist_items::create)
                .service(routes::checklist_items::update)
            )
        )
  })
  .bind("0.0.0.0:30083")?
  .run()
  .await
}
