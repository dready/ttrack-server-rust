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
extern crate env_logger;

mod endpoints;
mod repositories;
mod service;

use endpoints::user::list;
use service::AppState;

use actix_web::{http, middleware, server, App, HttpRequest, Responder};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use std::env;

fn greet(req: &HttpRequest<AppState>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    env::set_var(
        "DB_CONNECTION",
        "postgres://postgres:postgres@localhost:5432/ttrack",
    );
    let connection_string = env::var("DB_CONNECTION").expect(
        "Environment Variable DB_CONNECTION is missing for database connection configuration",
    );

    let manager = PostgresConnectionManager::new(connection_string, TlsMode::None)
        .expect("Cannot create postgres connection manager");
    let pool = r2d2::Pool::new(manager).expect("Cannot connect to database");

    server::new(move || {
        App::with_state(AppState { pool: pool.clone() })
            .prefix("/api")
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.f(greet))
            .resource("/users", |r| r.method(http::Method::GET).f(list))
            .resource("/{name}", |r| r.f(greet))
    }).bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}
