use serde::{Serialize, Deserialize};
use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::checklist_items;
use crate::db;

#[derive(AsChangeset, Serialize, Deserialize, Queryable)]
pub struct ChecklistItem {
  pub id: i32,
  pub name: String,
  pub completed: bool,
  pub checklist_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[table_name = "checklist_items"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable)]
pub struct NewChecklistItem {
  pub name: String,
  pub completed: bool,
  pub checklist_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl ChecklistItem {
    pub fn create(checklist_item: NewChecklistItem) -> Result<ChecklistItem, diesel::result::Error> {
      let connection = db::connection();

      diesel::insert_into(checklist_items::table)
        .values(&checklist_item)
        .get_result(&*connection)
    }

    pub fn read(checklist_id: i32) -> Result<Vec<ChecklistItem>, diesel::result::Error> {
      let connection = db::connection();

      checklist_items::table
        .filter(checklist_items::checklist_id.eq(checklist_id))
        // .order(checklists::order)
        .load::<ChecklistItem>(&*connection)
    }

    pub fn update(id: i32, checklist_item: ChecklistItem) -> Result<ChecklistItem, diesel::result::Error> {
      let connection = db::connection();

      diesel::update(
        checklist_items::table.filter(checklist_items::id.eq(id))
      )
        .set(&checklist_item)
        .get_result(&*connection)
    }

    pub fn find(id: i32) -> Result<ChecklistItem, diesel::result::Error> {
      let connection = db::connection();

      checklist_items::table
        .find(id)
        .first(&*connection)
    }
}
