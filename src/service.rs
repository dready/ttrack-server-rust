extern crate r2d2;
extern crate r2d2_postgres;

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

pub struct AppState {
    pub pool: Pool<PostgresConnectionManager>,
}
