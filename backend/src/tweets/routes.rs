use actix_web::web::{self, Json, Path};
use actix_web::HttpResponse;

use crate::constants::APPLICATION_JSON;
use crate::tweets::{TweetRequest, Tweets};

/// List 50 last tweets `/tweets`
#[get("/tweets")]
pub async fn list() -> HttpResponse {
    let tweets = Tweets::find_all().unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

/// Create a tweet `/tweets`
#[post("/tweets")]
pub async fn create(tweet_req: Json<TweetRequest>) -> HttpResponse {
    if let Some(message) = &tweet_req.message {
        Tweets::create(message.clone()).unwrap();
    } else {
        return HttpResponse::BadRequest().body("Message is required");
    }
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(tweet_req.to_tweet())
}

/// Find a tweet by its id `/tweets/{id}`
#[get("/tweets/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    let tweet = Tweets::find(path.0.parse().unwrap()).unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet)
}

/// Delete a tweet by its id `/tweets/{id}`
#[delete("/tweets/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    let deleted_tweet = Tweets::delete(path.0.parse().unwrap()).unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(deleted_tweet)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(create);
    config.service(get);
    config.service(delete);
}
