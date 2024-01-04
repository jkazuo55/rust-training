
use actix_web::{ get, post,delete, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Path;

async fn saludar() -> impl Responder {
    HttpResponse::Ok().body("holaº")
}

// api/tweets
#[get("/tweets")]
async fn get_tweets() -> HttpResponse {
    // get tweets
    let tweets = ["tweet 1 : hola", "tweet 2 : chao"];
    HttpResponse::Ok()
        .content_type("application/json")
        .json(tweets)
}

#[post("/tweets")]
async fn create_tweet() -> HttpResponse {
    // get tweets
    let new_tweet = "este es mi nuevo tweet";
    HttpResponse::Created() 
        .content_type("application/json")
        .json(new_tweet)
}

#[get("/tweets/{id}")]
async fn get_tweet_by_id(path:Path<(String,)>) -> HttpResponse {
    // get tweets
    let tweet = format!("este es el tweet {:?}", path.0);
    HttpResponse::Ok() 
        .content_type("application/json")
        .json(tweet)
}

#[get("/tweets/{id}/likes")]
async fn get_likes_by_tweet(path:Path<(String,)>) -> HttpResponse {
    // get tweets
    let likes = 100;
    HttpResponse::Ok() 
        .content_type("application/json")
        .json(likes)
}

#[post("/tweets/{id}/likes")]
async fn like_tweet(path:Path<(String,)>) -> HttpResponse {
    // get tweets
    let like = "ok";
    HttpResponse::Ok() 
        .content_type("application/json")
        .json(like)
}

#[delete("/tweets/{id}/likes")]
async fn remove_like(path:Path<(String,)>) -> HttpResponse {
    // get tweets
    HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(get_tweets)
        .service(create_tweet)
        .service(get_tweet_by_id)
        .service(remove_like)
        .service(like_tweet)
        .service(get_likes_by_tweet)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
