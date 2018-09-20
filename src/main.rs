extern crate actix_web;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::{http, server, App, Error, HttpRequest, HttpResponse, Json, Responder};
// use std::time::Instant;
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

#[derive(Serialize, Debug)]
struct User {
    id: i32,
    firstname: String,
    lastname: String,
    email: String,
}

impl Responder for User {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        // Create response and set content type
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

// pub struct DbExecutor(Connection);

// impl DbExecutor {
//     pub fn new(url: &str) -> DbExecutor {
//         DbExecutor(match Connection::connect(url, TlsMode::None) {
//             Ok(conn) => conn,
//             Err(err) => panic!("Error connecting to {} {:?}", url, err),
//         })
//     }
// }

struct AppState {
    pool: Pool<PostgresConnectionManager>,
}

// Response Error trait

fn users(req: &HttpRequest<AppState>) -> impl Responder {
    let db = req.state().pool.get().unwrap(); // <- get count

    // let sql = "SELECT * FROM users WHERE (usr_employment_start IS NULL OR usr_employment_start <= $1) AND (usr_employment_end IS NULL OR usr_employment_end >= $1)";
    // let now = Instant::now();
    let sql = "SELECT id, firstname, lastname, email FROM users";

    let mut u = Vec::new();
    for row in &db.query(sql, &[]).unwrap() {
        let user = User {
            id: row.get(0),
            firstname: row.get(1),
            lastname: row.get(2),
            email: row.get(3),
        };
        u.push(user);
    }
    Json(u)
}

// map().collect() -> Trait fromIterator

fn greet(req: &HttpRequest<AppState>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    let manager =
        PostgresConnectionManager::new("postgres://localhost:5432/ttrack", TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    // let db = DbExecutor::new("postgres://localhost:5432/ttrack");
    println!("Starting Server on http://127.0.0.1:8000/");
    server::new(|| {
        App::with_state(AppState { pool: pool.clone() })
            .prefix("/api")
            .resource("/", |r| r.f(greet))
            .resource("/users", |r| r.method(http::Method::GET).f(users))
            .resource("/{name}", |r| r.f(greet))
    }).bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
