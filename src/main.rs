extern crate actix_web;
extern crate postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use actix_web::{server, App, Error, HttpRequest, HttpResponse, Responder};
use postgres::{Connection, TlsMode};
use std::collections::HashMap;
use std::time::Instant;

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

pub struct DbExecutor(Connection);

impl DbExecutor {
    pub fn new(url: &str) -> DbExecutor {
        DbExecutor(match Connection::connect(url, TlsMode::None) {
            Ok(conn) => conn,
            Err(err) => panic!("Error connecting to {} {:?}", url, err),
        })
    }
}

fn users_hashmap(req: &HttpRequest, db: DbExecutor) -> impl Responder {
    let sql = "SELECT * FROM users WHERE (usr_employment_start IS NULL OR usr_employment_start <= $1) AND (usr_employment_end IS NULL OR usr_employment_end >= $1)";
    let now = Instant::now();
    let mut users = HashMap::new();
    for row in &db.0.query(sql, &[]).unwrap() {
        let user = User {
            id: row.get(0),
            firstname: row.get(1),
            lastname: row.get(2),
            email: row.get(3),
        };
        users.insert(user.id, user);
    }
}

fn users(req: &HttpRequest, db: DbExecutor) -> impl Responder {
    let sql = "SELECT id, firstname, lastname, email FROM users";

    &db.0.query(sql, &[]).unwrap().iter().map(|row| User {
        id: row.get(0),
        firstname: row.get(1),
        lastname: row.get(2),
        email: row.get(3),
    })
}

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    let db = DbExecutor::new("postgres://localhost:5432/ttrack");
    println!("Starting Server on http://127.0.0.1:8000/");
    server::new(|| {
        App::new()
            .prefix("/api")
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    }).bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
