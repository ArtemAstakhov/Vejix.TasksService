use std::ops::Deref;
use std::env;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, PooledConnection};

pub type MysqlPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> MysqlPool {
  let database_url = env::var("DATABASE_URL").unwrap();
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  Pool::new(manager).expect("Failed to create pool")
}

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}