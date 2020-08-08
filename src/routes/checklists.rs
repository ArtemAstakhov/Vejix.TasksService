use rocket_contrib::json::{Json, JsonValue};
use chrono::offset::Utc;

use crate::db;
use crate::models::checklist::{Checklist, NewChecklist};
use crate::user_inject;

#[derive(Deserialize)]
pub struct NewChecklistPayload {
  pub name: String,
}

#[get("/")]
pub fn read(
  connection: db::Connection,
  user_token: user_inject::UserToken
) -> Json<Vec<Checklist>> {
    Json(Checklist::read(user_token.id.unwrap(), &connection))
}

#[post("/", data = "<checklist>")]
pub fn create(
  checklist: Json<NewChecklistPayload>,
  connection: db::Connection,
  user_token: user_inject::UserToken
) -> Json<Checklist> {
    let insert = NewChecklist {
      name: checklist.name.clone(),
      user_id: user_token.id.unwrap(),
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
    };
    Json(Checklist::create(insert, &connection))
}

#[put("/<id>", data = "<checklist>")]
pub fn update(
  id: i32,
  checklist: Json<Checklist>,
  connection: db::Connection,
  user_token: user_inject::UserToken,
) -> Json<Checklist> {
  let update = Checklist { id, ..checklist.into_inner() };
  Json(Checklist::update(id, user_token.id.unwrap(), update, &connection))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: db::Connection) -> Json<JsonValue> {
    Json(json!({
        "success": Checklist::delete(id, &connection),
        "id": id,
    }))
}
