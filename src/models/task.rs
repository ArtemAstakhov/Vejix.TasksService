use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::NaiveDateTime;

use crate::schema::tasks;
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
    pub fn create(task: NewTask, connection: &PgConnection) -> Task {
      diesel::insert_into(tasks::table)
        .values(&task)
        .execute(connection)
        .expect("Error creating new task");

        tasks::table.order(tasks::id.desc()).first(connection).unwrap()
    }

    pub fn read(user_id: i32, connection: &PgConnection) -> Vec<Task> {
      tasks::table
        .filter(tasks::user_id.eq(user_id))
        .order(tasks::order)
        .load::<Task>(connection)
        .unwrap()
    }

    pub fn update(id: i32, user_id: i32, task: Task, connection: &PgConnection) -> Task {
        diesel::update(
          tasks::table.filter(tasks::user_id.eq(user_id))
            .filter(tasks::id.eq(id))
        )
          .set(&task)
          .execute(connection)
          .expect("Error updating task");

        // let uncompleted_tasks: i64 = tasks::table
        //   .filter(tasks::user_id.eq(user_id))
        //   .filter(tasks::completed.eq(false))
        //   .count()
        //   .get_result(connection)
        //   .unwrap();

        // if uncompleted_tasks == 0 {
        //   let mut user = User::find(user_id, connection);
        //   let increase: i32 = 100;

        //   user.experience = user.experience + increase;
        //   User::update(user, connection);
        // }

        Task::find(id, connection)
    }

    pub fn find(id: i32, connection: &PgConnection) -> Task {
      tasks::table
        .find(id)
        .first(connection)
        .unwrap()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
      diesel::delete(tasks::table.find(id)).execute(connection).is_ok()
    }

    pub fn reorder(id: i32, new_order: i32, connection: &PgConnection) -> bool {
      diesel::update(tasks::table.find(id))
        .set(tasks::order.eq(new_order))
        .execute(connection)
        .is_ok()
    }
}
