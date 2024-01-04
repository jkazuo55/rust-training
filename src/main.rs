use actix_web::{App, HttpServer};

mod likes;
mod tweets;
mod constants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(tweets::get_tweets)
            .service(tweets::create_tweet)
            .service(tweets::get_tweet_by_id)
            .service(likes::remove_like)
            .service(likes::like_tweet)
            .service(likes::get_likes_by_tweet)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
