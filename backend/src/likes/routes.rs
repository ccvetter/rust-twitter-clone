use actix_web::web::{self, Path};
use actix_web::HttpResponse;
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::likes::Likes;

/// List last 50 likes from a tweet `/tweets/{id}/likes`
#[get("/tweets/{id}/likes")]
pub async fn list(path: Path<(Uuid,)>) -> HttpResponse {
    let likes = Likes::find_all(path.0).unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(likes)
}

/// Add one like to a tweet `/tweets/{id}/likes`
#[post("/tweets/{id}/likes")]
pub async fn plus_one(path: Path<(Uuid,)>) -> HttpResponse {
    let like = Likes::create(path.0).unwrap();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(like)
}

/// Remove one like from a tweet `/tweets/{id}/likes`
#[delete("/tweets/{id}/likes")]
pub async fn minus_one(path: Path<(Uuid,)>) -> HttpResponse {
    Likes::delete(path.0).unwrap();
    HttpResponse::NoContent().finish()
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(plus_one);
    config.service(minus_one);
}
