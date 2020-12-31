use std::env;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection};
use lazy_static::lazy_static;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
  static ref POOL: PgPool = {
    let database_url = env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
      .max_size(10)
      .min_idle(Some(5))
      .build(manager)
      .expect("Failed to create pool");
  
    pool
  };
}

pub fn init() {
  let conn = connection();
  embedded_migrations::run(&*conn).unwrap();
}

pub fn connection() -> DbConnection {
  POOL.get().expect("Failed getting db connection")
}
