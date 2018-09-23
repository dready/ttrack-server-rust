use actix_web::{HttpRequest, Json, Responder};
use repositories::user_repository::list_active;
use service::AppState;

pub fn list(req: &HttpRequest<AppState>) -> impl Responder {
    let u = list_active(&req.state().pool).unwrap();
    Json(u)
}
