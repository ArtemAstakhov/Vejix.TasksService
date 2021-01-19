use serde::{Deserialize};
use serde_json::json;
use chrono::offset::Utc;
use actix_web::{HttpResponse, Result, get, http, post, web, put, delete};

use crate::dto::error::{Errors, ServerError};
use crate::models::task::{Task, NewTask};
use crate::user_inject::{self, get_user_id};

#[derive(Deserialize)]
pub struct NewTaskPayload {
  pub title: String,
  pub order: i32,
  pub task_list_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct ReorderTasksPayload {
  pub id: i32,
  pub order: i32,
}

#[get("/")]
pub async fn read(
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  let result = Task::read(user_id);

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

#[post("/")]
pub async fn create(
  task: web::Json<NewTaskPayload>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  let insert = NewTask {
    user_id,
    date: Utc::now().naive_utc(),
    created_at: Utc::now().naive_utc(),
    updated_at: Utc::now().naive_utc(),
    title: task.title.clone(),
    order: task.order,
    completed: false,
    deadline: None,
    tag: None,
    task_list_id: task.task_list_id.clone(),
  };
  let result = Task::create(insert);

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
  task: web::Json<Task>,
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
  let update = Task { ..task.into_inner() };
  let result = Task::update(user_id, update);

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

#[put("/{id}/complete")]
pub async fn complete(
  params: web::Path<UpdateTaskParams>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  let result = Task::complete(user_id, params.id);

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

#[put("/{id}/uncomplete")]
pub async fn uncomplete(
  params: web::Path<UpdateTaskParams>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  let result = Task::uncomplete(user_id, params.id);

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
pub struct DeleteTaskParams {
  id: i32,
}

#[delete("/{id}")]
pub async fn delete(
  params: web::Path<DeleteTaskParams>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let _ = get_user_id(user_token).unwrap();
  let result = Task::delete(params.id);
  
  Ok(HttpResponse::Ok().json(json!({
    "success": result,
    "id": params.id,
})))
}

#[put("/reorder")]
pub async fn reorder(
  tasks: web::Json<Vec<ReorderTasksPayload>>,
  user_token: user_inject::UserToken,
) -> Result<HttpResponse, Errors> {
  let user_id = get_user_id(user_token).unwrap();
  for task in tasks.iter() {
    Task::reorder(task.id, task.order);
  }

  let result = Task::read(user_id);

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