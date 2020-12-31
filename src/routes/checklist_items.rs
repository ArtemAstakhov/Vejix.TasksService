use serde::{Deserialize};
use actix_web::{HttpResponse, Result, get, http, post, web, put};
use chrono::offset::Utc;

use crate::dto::error::{Errors, ServerError};
use crate::models::checklist_item::{ChecklistItem, NewChecklistItem};
use crate::user_inject::{self, get_user_id};

#[derive(Deserialize)]
pub struct ReadChecklistItemsQuery {
  pub checklist_id: i32,
}

#[get("/")]
pub async fn read(
  query: web::Query<ReadChecklistItemsQuery>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let _ = get_user_id(user_token).unwrap();
  let result = ChecklistItem::read(query.checklist_id);
  

  match result {
    Ok(checklists) => {
      Ok(HttpResponse::Ok().json(checklists))
    }
    _ => {
      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![]
      })
    }
  }
}

#[derive(Deserialize)]
pub struct NewChecklistItemPayload {
  pub name: String,
  pub checklist_id: i32,
}

#[post("/")]
pub async fn create(
  checklist_item: web::Json<NewChecklistItemPayload>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let _ = get_user_id(user_token).unwrap();

  let insert = NewChecklistItem {
    name: checklist_item.name.clone(),
    checklist_id: checklist_item.checklist_id,
    completed: false,
    created_at: Utc::now().naive_utc(),
    updated_at: Utc::now().naive_utc(),
  };
  let result = ChecklistItem::create(insert);

  match result {
    Ok(checklist) => {
      Ok(HttpResponse::Ok().json(checklist))
    },
    _ => {
      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![]
      })
    },
  }
}

#[derive(Deserialize)]
pub struct UpdateChecklistItemParams {
  id: i32,
}

#[put("/{id}")]
pub async fn update(
  params: web::Path<UpdateChecklistItemParams>,
  checklist_item: web::Json<ChecklistItem>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  if params.id != checklist_item.id {
    return Err(Errors {
      status: http::StatusCode::BAD_REQUEST,
      errors: vec![ServerError {
        code: "ids_conflict".to_string(),
        message: "Request ids don't match".to_string(),
        field: None,
      }],
    });
  }

  let _ = get_user_id(user_token).unwrap();

  let update = ChecklistItem { id: checklist_item.id, ..checklist_item.into_inner() };
  let result = ChecklistItem::update(params.id, update);

  match result {
    Ok(checklist) => {
      Ok(HttpResponse::Ok().json(checklist))
    },
    _ => {
      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![]
      })
    },
  }
}

