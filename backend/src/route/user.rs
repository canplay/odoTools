use crate::db;
use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse, Result};
use mongodb::Database;
use serde_json::{json, Value};

#[post("/user/register")]
async fn register(req: web::Json<Value>, db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let r = db::user::get_user(req["username"].as_str().unwrap(), &db)
        .await
        .unwrap();
    Ok(HttpResponse::Ok().json(r))
}
