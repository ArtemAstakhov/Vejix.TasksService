use serde::{Deserialize};
use serde_json::json;
use chrono::offset::Utc;
use actix_web::{HttpResponse, Result, get, http, post, web, put, delete};

use crate::dto::error::{Errors, ServerError};
use crate::models::tasks_list::{TaskList, NewTaskList};
use crate::user_inject::{self, get_user_id};

#[get("/")]
pub async fn read(
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  let result = TaskList::read(user_id);

  match result {
    Ok(tasks) => {
      Ok(HttpResponse::Ok().json(tasks))
    }
    Err(_) => {
      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![],
      })
    }
  }
}

#[derive(Deserialize)]
pub struct NewTaskListPayload {
  pub name: String,
}

#[post("/")]
pub async fn create(
  task_list: web::Json<NewTaskListPayload>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  let insert = NewTaskList {
    user_id,
    created_at: Utc::now().naive_utc(),
    updated_at: Utc::now().naive_utc(),
    name: task_list.name.clone(),
  };
  let result = TaskList::create(insert);

  match result {
    Ok(task) => {
      Ok(HttpResponse::Ok().json(task))
    }
    Err(_) => {
      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![],
      })
    }
  }
}

#[derive(Deserialize)]
pub struct UpdateTaskParams {
  id: i32,
}

#[put("/{id}")]
pub async fn update(
  params: web::Path<UpdateTaskParams>,
  task: web::Json<TaskList>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  if params.id != task.id {
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
  let update = TaskList { ..task.into_inner() };
  let result = TaskList::update(user_id, update);

  match result {
    Ok(task) => {
      Ok(HttpResponse::Ok().json(task))
    }
    Err(_) => {
      Err(Errors {
        status: http::StatusCode::INTERNAL_SERVER_ERROR,
        errors: vec![],
      })
    }
  }
}

#[derive(Deserialize)]
pub struct DeleteTaskListParams {
  id: i32,
}

#[delete("/{id}")]
pub async fn delete(
  params: web::Path<DeleteTaskListParams>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let _ = get_user_id(user_token).unwrap();
  let result = TaskList::delete(params.id);
  
  Ok(HttpResponse::Ok().json(json!({
    "success": result,
    "id": params.id,
})))
}
