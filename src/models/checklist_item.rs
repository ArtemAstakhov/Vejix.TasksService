use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::NaiveDateTime;

use crate::schema::checklist_items;

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
    pub fn create(checklist_item: NewChecklistItem, connection: &PgConnection) -> ChecklistItem {
      diesel::insert_into(checklist_items::table)
        .values(&checklist_item)
        .execute(connection)
        .expect("Error creating new checklist item");

        checklist_items::table.order(checklist_items::id.desc()).first(connection).unwrap()
    }

    pub fn read(checklist_id: i32, connection: &PgConnection) -> Vec<ChecklistItem> {
      checklist_items::table
        .filter(checklist_items::checklist_id.eq(checklist_id))
        // .order(checklists::order)
        .load::<ChecklistItem>(connection)
        .unwrap()
    }

    pub fn update(id: i32, checklist_item: ChecklistItem, connection: &PgConnection) -> ChecklistItem {
      diesel::update(
        checklist_items::table.filter(checklist_items::id.eq(id))
      )
        .set(&checklist_item)
        .execute(connection)
        .expect("Error updating item");

        ChecklistItem::find(id, connection)
    }

    pub fn find(id: i32, connection: &PgConnection) -> ChecklistItem {
      checklist_items::table
        .find(id)
        .first(connection)
        .unwrap()
    }
}
