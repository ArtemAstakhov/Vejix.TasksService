use serde::{Serialize, Deserialize};
use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::tasks;
use crate::db;
// use super::user::User;

#[derive(AsChangeset, Serialize, Deserialize, Queryable)]
pub struct Task {
  pub id: i32,
  pub date: NaiveDateTime,
  pub title: String,
  pub completed: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deadline: Option<NaiveDateTime>,
  pub tag: Option<String>,
  pub user_id: i32,
  pub order: i32,
}

#[table_name = "tasks"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable)]
pub struct NewTask {
  pub date: NaiveDateTime,
  pub title: String,
  pub completed: bool,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub deadline: Option<NaiveDateTime>,
  pub tag: Option<String>,
  pub user_id: i32,
  pub order: i32,
}

impl Task {
    pub fn create(task: NewTask) -> Result<Task, diesel::result::Error> {
      let connection = db::connection();

      diesel::insert_into(tasks::table)
        .values(&task)
        .get_result(&*connection)
    }

    pub fn read(user_id: i32) -> Result<Vec<Task>, diesel::result::Error> {
      let connection = db::connection();

      tasks::table
        .filter(tasks::user_id.eq(user_id))
        .order(tasks::order)
        .load::<Task>(&*connection)
    }

    pub fn update(user_id: i32, task: Task) -> Result<Task, diesel::result::Error> {
      let connection = db::connection();

      diesel::update(
        tasks::table
          .filter(tasks::user_id.eq(user_id))
          .filter(tasks::id.eq(task.id))
      )
        .set(&task)
        .get_result(&*connection)
    }

    pub fn complete(user_id: i32, task_id: i32) -> Result<Task, diesel::result::Error> {
      let connection = db::connection();

      diesel::update(
        tasks::table
          .filter(tasks::user_id.eq(user_id))
          .filter(tasks::id.eq(task_id))
      )
        .set(tasks::completed.eq(true))
        .get_result(&*connection)
    }

    pub fn uncomplete(user_id: i32, task_id: i32) -> Result<Task, diesel::result::Error> {
      let connection = db::connection();

      diesel::update(
        tasks::table
          .filter(tasks::user_id.eq(user_id))
          .filter(tasks::id.eq(task_id))
      )
        .set(tasks::completed.eq(false))
        .get_result(&*connection)
    }

    pub fn find(id: i32) -> Result<Task, diesel::result::Error> {
      let connection = db::connection();

      tasks::table
        .find(id)
        .first(&*connection)
    }

    pub fn delete(id: i32) -> bool {
      let connection = db::connection();

      diesel::delete(tasks::table.find(id)).execute(&*connection).is_ok()
    }

    pub fn reorder(id: i32, new_order: i32) -> bool {
      let connection = db::connection();

      diesel::update(tasks::table.find(id))
        .set(tasks::order.eq(new_order))
        .execute(&*connection)
        .is_ok()
    }
}
