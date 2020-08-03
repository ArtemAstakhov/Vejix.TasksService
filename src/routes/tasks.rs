use rocket_contrib::json::{Json, JsonValue};
use chrono::offset::Utc;

use crate::db;
use crate::models::task::{Task, NewTask};
use crate::user_inject;

#[derive(Deserialize)]
pub struct NewTaskPayload {
  pub title: String,
  pub order: i32,
}

#[derive(Deserialize)]
pub struct ReorderTasksPayload {
  pub id: i32,
  pub order: i32,
}

#[get("/")]
pub fn read(
  connection: db::Connection,
  user_token: user_inject::UserToken
) -> Json<Vec<Task>> {
    Json(Task::read(user_token.id.unwrap(), &connection))
}

#[post("/", data = "<task>")]
pub fn create(
  task: Json<NewTaskPayload>,
  connection: db::Connection,
  user_token: user_inject::UserToken
) -> Json<Task> {
    let insert = NewTask {
      user_id: user_token.id.unwrap(),
      date: Utc::now().naive_utc(),
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
      title: task.title.clone(),
      order: task.order,
      completed: false,
      deadline: None,
      tag: None,
    };
    Json(Task::create(insert, &connection))
}

#[put("/<id>", data = "<task>")]
pub fn update(
  id: i32,
  task: Json<Task>,
  connection: db::Connection,
  user_token: user_inject::UserToken
) -> Json<Task> {
  let update = Task { id, ..task.into_inner() };
  Json(Task::update(id, user_token.id.unwrap(), update, &connection))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Task::delete(id, &connection),
        "id": id,
    }))
}

#[put("/reorder", data = "<tasks>")]
pub fn reorder(
  tasks: Json<Vec<ReorderTasksPayload>>,
  connection: db::Connection,
  user_token: user_inject::UserToken
) -> Json<Vec<Task>> {
  for task in tasks.iter() {
    Task::reorder(task.id, task.order, &connection);
  }

  Json(Task::read(user_token.id.unwrap(), &connection))
}