use std::env;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, Validation};
use actix_web::{dev, Error, HttpRequest, FromRequest, http};
use futures::future::{ok, Ready};

use crate::dto::error::Errors;

#[derive(Serialize, Deserialize)]
pub struct UserToken {
  pub id: Option<i32>,
  pub username: Option<String>,
}

fn get_token_data(token: &str) -> UserToken {
  decode::<UserToken>(
    &token,
    env::var("TOKEN_SECRET").unwrap().as_ref(),
    &Validation {validate_exp: false, ..Default::default()}).unwrap().claims
}

impl FromRequest for UserToken {
  type Error = Error;
  type Future = Ready<Result<Self, Self::Error>>;
  type Config = ();

  fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
    let authorization = req.headers().get("Authorization");

    let token = match authorization {
      Some(h) => get_token_data(h.to_str().unwrap()),
      _ => UserToken { id: None, username: None }
    };

    ok(token)
  }
}

pub fn get_user_id(token: UserToken) -> Result<i32, Errors> {
  match token.id {
    Some(id) => {
      Ok(id)
    },
    None => {
      Err(Errors {
        status: http::StatusCode::UNAUTHORIZED,
        errors: vec![]
      })
    }
  }
}

