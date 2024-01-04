use crate::constants::APPLICATION_JSON;
use actix_web::web::Path;
use actix_web::{ get, post, HttpResponse};

// api/tweets
#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    // get tweets
    let tweets = ["tweet 1 : hola", "tweet 2 : chao"];
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweets)
}

#[post("/tweets")]
pub async fn create_tweet() -> HttpResponse {
    // get tweets
    let new_tweet = "este es mi nuevo tweet";
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    // get tweets
    let tweet = format!("este es el tweet {:?}", path.0);
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet)
}
