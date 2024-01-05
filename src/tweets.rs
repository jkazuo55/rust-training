use super::schema::tweets;
use crate::constants::APPLICATION_JSON;
use actix_web::web::{Data, Path};
use chrono::{NaiveDateTime, Utc};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl};
use uuid::Uuid;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use actix_web::{ get, post, HttpResponse};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

#[get("/tweets")]
pub async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> HttpResponse {
    use crate::schema::tweets::dsl::*;

    let mut conn = pool
        .get()
        .expect(" No pude obtener conexión a la base de datos");
    let result = tweets
        .order(created_at.desc())
        .limit(10) // sería bueno parametrizar este valor
        .load::<Tweet>(&mut conn);

    let response = match result {
        Ok(tws) => tws,
        Err(_) => vec![],
    };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(response)
}

#[post("/tweets")]
pub async fn create_tweet(req_body: String,pool: Data<Pool<ConnectionManager<PgConnection>>>,) -> HttpResponse {
    let new_tweet = Tweet::new(req_body);
    let mut conn = pool
        .get()
        .expect(" No pude obtener conexión a la base de datos");
    diesel::insert_into(tweets::table)
        .values(&new_tweet)
        .execute(&mut conn)
        .expect("Error al insertar tweet");

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    // get tweets
    let tweet = format!("este es el tweet {:?}", path.0);
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(tweet)
}
