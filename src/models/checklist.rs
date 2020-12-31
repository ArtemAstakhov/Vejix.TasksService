use serde::{Serialize, Deserialize};
use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::{checklists, checklist_items};
use crate::db;

#[derive(AsChangeset, Serialize, Deserialize, Queryable)]
pub struct Checklist {
  pub id: i32,
  pub name: String,
  pub user_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[table_name = "checklists"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable)]
pub struct NewChecklist {
  pub name: String,
  pub user_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl Checklist {
    pub fn create(checklist: NewChecklist) -> Result<Checklist, diesel::result::Error> {
      let connection = db::connection();

      diesel::insert_into(checklists::table)
        .values(&checklist)
        .get_result(&*connection)
    }

    pub fn read(user_id: i32) -> Result<Vec<Checklist>, diesel::result::Error> {
      let connection = db::connection();

      checklists::table
        .filter(checklists::user_id.eq(user_id))
        // .order(checklists::order)
        .load::<Checklist>(&*connection)
    }

    pub fn update(id: i32, user_id: i32, checklist: Checklist) -> Result<Checklist, diesel::result::Error> {
      let connection = db::connection();

      diesel::update(
        checklists::table.filter(checklists::user_id.eq(user_id))
          .filter(checklists::id.eq(id))
      )
        .set(&checklist)
        .get_result(&*connection)
    }

    pub fn find(id: i32) -> Result<Checklist, diesel::result::Error> {
      let connection = db::connection();

      checklists::table
        .find(id)
        .first(&*connection)
    }

    pub fn delete(id: i32) -> bool {
      let connection = db::connection();
      // TODO
      diesel::delete(checklists::table.find(id)).execute(&*connection).unwrap();
      diesel::delete(checklist_items::table.filter(checklist_items::checklist_id.eq(id))).execute(&*connection).is_ok()
    }
}
