use rocket_contrib::json::{Json};
use chrono::offset::Utc;

use crate::db;
use crate::models::checklist_item::{ChecklistItem, NewChecklistItem};

#[derive(Deserialize)]
pub struct NewChecklistItemPayload {
  pub name: String,
  pub checklist_id: i32,
}

#[get("/?<checklist_id>")]
pub fn read(
  connection: db::Connection,
  checklist_id: i32,
) -> Json<Vec<ChecklistItem>> {
    Json(ChecklistItem::read(checklist_id, &connection))
}

#[post("/", data = "<checklist_item>")]
pub fn create(
  checklist_item: Json<NewChecklistItemPayload>,
  connection: db::Connection,
) -> Json<ChecklistItem> {
    let insert = NewChecklistItem {
      name: checklist_item.name.clone(),
      checklist_id: checklist_item.checklist_id,
      completed: false,
      created_at: Utc::now().naive_utc(),
      updated_at: Utc::now().naive_utc(),
    };
    Json(ChecklistItem::create(insert, &connection))
}

#[put("/<id>", data = "<checklist_item>")]
pub fn update(
  id: i32,
  checklist_item: Json<ChecklistItem>,
  connection: db::Connection,
) -> Json<ChecklistItem> {
  let update = ChecklistItem { id, ..checklist_item.into_inner() };
  Json(ChecklistItem::update(id, update, &connection))
}

