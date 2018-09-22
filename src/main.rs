extern crate actix_web;
extern crate postgres;
#[macro_use]
extern crate postgres_derive;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod repositories;

use repositories::user::User;

use actix_web::{http, server, App, HttpRequest, Json, Responder};
use r2d2::Pool;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

struct AppState {
    pool: Pool<PostgresConnectionManager>,
}

// Response Error trait

fn users(req: &HttpRequest<AppState>) -> impl Responder {
    let u = User::list_active(&req.state().pool).unwrap();
    Json(u)
}

// map().collect() -> Trait fromIterator

fn greet(req: &HttpRequest<AppState>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    let manager = PostgresConnectionManager::new(
        "postgres://postgres:postgres@localhost:5432/ttrack",
        TlsMode::None,
    ).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    // let db = DbExecutor::new("postgres://localhost:5432/ttrack");
    println!("Starting Server on http://127.0.0.1:8000/");
    server::new(move || {
        App::with_state(AppState { pool: pool.clone() })
            .prefix("/api")
            .resource("/", |r| r.f(greet))
            .resource("/users", |r| r.method(http::Method::GET).f(users))
            .resource("/{name}", |r| r.f(greet))
    }).bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
