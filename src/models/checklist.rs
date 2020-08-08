use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::NaiveDateTime;

use crate::schema::{checklists, checklist_items};

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
    pub fn create(checklist: NewChecklist, connection: &PgConnection) -> Checklist {
      diesel::insert_into(checklists::table)
        .values(&checklist)
        .execute(connection)
        .expect("Error creating new checklist");

        checklists::table.order(checklists::id.desc()).first(connection).unwrap()
    }

    pub fn read(user_id: i32, connection: &PgConnection) -> Vec<Checklist> {
      checklists::table
        .filter(checklists::user_id.eq(user_id))
        // .order(checklists::order)
        .load::<Checklist>(connection)
        .unwrap()
    }

    pub fn update(id: i32, user_id: i32, checklist: Checklist, connection: &PgConnection) -> Checklist {
        diesel::update(
          checklists::table.filter(checklists::user_id.eq(user_id))
            .filter(checklists::id.eq(id))
        )
          .set(&checklist)
          .execute(connection)
          .expect("Error updating checklist");

          Checklist::find(id, connection)
    }

    pub fn find(id: i32, connection: &PgConnection) -> Checklist {
      checklists::table
        .find(id)
        .first(connection)
        .unwrap()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
      diesel::delete(checklists::table.find(id)).execute(connection).unwrap();
      diesel::delete(checklist_items::table.filter(checklist_items::checklist_id.eq(id))).execute(connection).is_ok()
    }
}
