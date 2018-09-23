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

fn greet(req: &HttpRequest<AppState>) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = PostgresConnectionManager::new(
        "postgres://postgres:postgres@localhost:5432/ttrack",
        TlsMode::None,
    ).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

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
