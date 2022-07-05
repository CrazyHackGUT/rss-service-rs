use std::env::var;
use diesel::{Connection, PgConnection};
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub(crate) type DbConnection = PgConnection;
pub(crate) type DbConnectionManager = ConnectionManager<DbConnection>;
pub(crate) type DbPool = Pool<DbConnectionManager>;

pub(crate) fn connection() -> DbConnection {
    Connection::establish(url().as_str()).unwrap()
}

pub(crate) fn connection_manager() -> DbConnectionManager {
    ConnectionManager::new(url())
}

pub(crate) fn pool() -> DbPool {
    Pool::builder()
        .build(connection_manager())
        .unwrap()
}

fn url() -> String {
    var("DATABASE_URL").expect("DATABASE_URL is not set in environment variables")
}
