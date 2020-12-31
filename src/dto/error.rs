use serde::{Serialize};
use derive_more::{Display, Error};
use actix_web::{error, http, HttpResponse, dev::HttpResponseBuilder};

#[derive(Debug, Display, Error, Serialize)]
#[display(fmt = "Error: {}", message)]
pub struct ServerError {
  pub message: String,
  pub code: String,
  pub field: Option<&'static str>,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Error")]
pub struct Errors {
  pub status: http::StatusCode,
  pub errors: Vec<ServerError>,
}

impl error::ResponseError for Errors {
  fn error_response(&self) -> HttpResponse {
    let errors = self.errors.as_slice();

    HttpResponseBuilder::new(self.status)
        .set_header(http::header::CONTENT_TYPE, "application/json; charset=utf-8")
        .json(errors)
  }
}
