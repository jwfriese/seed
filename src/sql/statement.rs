use diesel::{PgConnection, RunQueryDsl, sql_query};
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct Statement {
    text: String,
}

pub type StatementError = String;

impl Statement {
    pub fn new(text: &str) -> Statement {
        Statement {
            text: String::from(text)
        }
    }

    pub fn run(&self, conn: &PooledConnection<ConnectionManager<PgConnection>>) -> Option<StatementError> {
        let result = sql_query(&self.text).execute(conn);
        match result {
            Ok(count) => {
                if count == 1 {
                    None
                } else {
                    Some(format!("Inserted an unexpected number of rows: {}", count))
                }
            }
            Err(err) => {
                Some(err.to_string())
            }
        }
    }
}