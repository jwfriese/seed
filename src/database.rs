use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::Config;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_database(config: &Config) -> Pool {
    let connection_string = format!(
        "postgres://{}:{}@{}/{}?sslmode=disable",
        config.db_user,
        config.db_password,
        config.db_host,
        config.db_name,
    );

    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
