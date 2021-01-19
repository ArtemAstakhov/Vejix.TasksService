use serde::{Serialize, Deserialize};
use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use crate::schema::tasks_lists;
use crate::db;

#[table_name = "tasks_lists"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable)]
pub struct TaskList {
  pub id: i32,
  pub name: String,
  pub user_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[table_name = "tasks_lists"]
#[derive(AsChangeset, Serialize, Deserialize, Insertable)]
pub struct NewTaskList {
  pub name: String,
  pub user_id: i32,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl TaskList {
    pub fn create(task_list: NewTaskList) -> Result<TaskList, diesel::result::Error> {
      let connection = db::connection();

      diesel::insert_into(tasks_lists::table)
        .values(&task_list)
        .get_result(&*connection)
    }

    pub fn read(user_id: i32) -> Result<Vec<TaskList>, diesel::result::Error> {
      let connection = db::connection();

      tasks_lists::table
        .filter(tasks_lists::user_id.eq(user_id))
        .load::<TaskList>(&*connection)
    }

    pub fn update(user_id: i32, task_list: TaskList) -> Result<TaskList, diesel::result::Error> {
      let connection = db::connection();

      diesel::update(
        tasks_lists::table
          .filter(tasks_lists::user_id.eq(user_id))
          .filter(tasks_lists::id.eq(task_list.id))
      )
        .set(&task_list)
        .get_result(&*connection)
    }

    pub fn find(id: i32) -> Result<TaskList, diesel::result::Error> {
      let connection = db::connection();

      tasks_lists::table
        .find(id)
        .first(&*connection)
    }

    pub fn delete(id: i32) -> bool {
      let connection = db::connection();

      diesel::delete(tasks_lists::table.find(id)).execute(&*connection).is_ok()
    }
}
