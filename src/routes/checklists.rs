use serde::{Deserialize};
use log::{info, warn};
use actix_web::{HttpResponse, Result, get, http, post, web, put, delete};
use chrono::offset::Utc;
use serde_json::json;

use crate::dto::error::ServerError;
use crate::dto::error::Errors;
use crate::models::checklist::{Checklist, NewChecklist};
use crate::user_inject::{self, get_user_id};

#[derive(Deserialize)]
pub struct NewChecklistPayload {
  pub name: String,
}

#[get("/")]
pub async fn read(
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();

  info!("Read checklists. User: {}", user_id);

  let result = Checklist::read(user_id);
  
  match result {
    Ok(checklists) => {
      Ok(HttpResponse::Ok().json(checklists))
    }
    Err(e) => {
      warn!("Read checklists error: {}", e);

      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![
          ServerError {
            message: e.to_string(),
            code: "database_error".to_string(),
            field: None,
          },
        ],
      })
    }
  }
}

#[post("/")]
pub async fn create(
  checklist: web::Json<NewChecklistPayload>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();

  info!("Create checklist. User: {}", user_id);

  let insert = NewChecklist {
    name: checklist.name.clone(),
    user_id,
    created_at: Utc::now().naive_utc(),
    updated_at: Utc::now().naive_utc(),
  };
  let result = Checklist::create(insert);

  match result {
    Ok(checklist) => {
      Ok(HttpResponse::Ok().json(checklist))
    }
    Err(e) => {
      warn!("Create checklists error: {}", e);

      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![
          ServerError {
            message: e.to_string(),
            code: "database_error".to_string(),
            field: None,
          },
        ],
      })
    }
  }
}

#[derive(Deserialize)]
pub struct UpdateChecklistParams {
  id: i32,
}

#[put("/{id}")]
pub async fn update(
  params: web::Path<UpdateChecklistParams>,
  checklist: web::Json<Checklist>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  if params.id != checklist.id {
    return Err(Errors {
      status: http::StatusCode::BAD_REQUEST,
      errors: vec![ServerError {
        code: "ids_conflict".to_string(),
        message: "Request ids don't match".to_string(),
        field: None,
      }],
    });
  }

  let user_id = get_user_id(user_token).unwrap();
  let update = Checklist { id: params.id, ..checklist.into_inner() };
  let result = Checklist::update(params.id, user_id, update);

  match result {
    Ok(checklist) => {
      Ok(HttpResponse::Ok().json(checklist))
    }
    Err(e) => {
      warn!("Create checklists error: {}", e);

      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![
          ServerError {
            message: e.to_string(),
            code: "database_error".to_string(),
            field: None,
          },
        ],
      })
    }
  }
}

#[derive(Deserialize)]
pub struct DeleteChecklistParams {
  id: i32,
}

#[delete("/{id}")]
pub async fn delete(
  params: web::Path<DeleteChecklistParams>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let _ = get_user_id(user_token).unwrap();

  let result = Checklist::delete(params.id);

  Ok(HttpResponse::Ok().json(json!({
    "success": result,
    "id": params.id,
  })))
}
