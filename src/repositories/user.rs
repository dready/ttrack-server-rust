extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

#[derive(Serialize, Debug, FromSql)]
pub struct User {
    #[postgres(name = "usr_id")]
    id: i32,
    #[postgres(name = "usr_firstname")]
    firstname: String,
    #[postgres(name = "usr_lastname")]
    lastname: String,
    #[postgres(name = "usr_email")]
    email: String,
}

#[derive(Debug)]
pub enum Error {
    ConnectionError(r2d2::Error),
    DBError(postgres::Error),
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::ConnectionError(e)
    }
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Self {
        Error::DBError(e)
    }
}

impl User {
    pub fn list_active(conn: &Pool<PostgresConnectionManager>) -> Result<Vec<User>, Error> {
        let db = conn.get()?;

        let sql = "SELECT * FROM users WHERE (usr_employment_start IS NULL OR usr_employment_start <= now()) AND (usr_employment_end IS NULL OR usr_employment_end >= now())";

        let mut users = Vec::new();
        let rows = &db.query(sql, &[]).expect("Failed to select active users");
        for row in rows {
            let user = User {
                id: row.get("usr_id"),
                firstname: row.get("usr_firstname"),
                lastname: row.get("usr_lastname"),
                email: row.get("usr_email"),
            };
            users.push(user);
        }

        Ok(users)
    }
}
