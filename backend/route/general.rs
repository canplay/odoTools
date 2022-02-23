use actix_session::Session;
use actix_web::{post, web, Error, HttpResponse, Result};
use blake2::{Blake2b512, Digest};
use chrono::Local;
use serde_json::{json, Value};

#[post("/login")]
async fn login(
    req: web::Json<Value>,
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("ok"))
}
