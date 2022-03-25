use crate::db;
use actix_web::{get, post, web, Error, HttpResponse, Result};
use actix_web_grants::proc_macro::has_permissions;
use mongodb::{bson::oid, Database};
use serde_json::{json, Value};

#[get("/website/news")]
async fn news(db: web::Data<Vec<Database>>) -> Result<HttpResponse, Error> {
    let ret = db::website::get_news("", &db[1]).await;

    if ret.is_err() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 1i64,
        "msg": ret.unwrap()
    })))
}

#[get("/website/carousel")]
async fn carousel(db: web::Data<Vec<Database>>) -> Result<HttpResponse, Error> {
    let ret = db::website::get_carousel("", &db[1]).await;

    if ret.is_err() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 1i64,
        "msg": ret.unwrap()
    })))
}

#[get("/website/downloads")]
async fn downloads(db: web::Data<Vec<Database>>) -> Result<HttpResponse, Error> {
    let ret = db::website::get_downloads("", &db[1]).await;

    if ret.is_err() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 1i64,
        "msg": ret.unwrap()
    })))
}

#[get("/website/shop")]
async fn shop(db: web::Data<Vec<Database>>) -> Result<HttpResponse, Error> {
    let ret = db::website::get_shop("", &db[1]).await;

    if ret.is_err() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 1i64,
        "msg": ret.unwrap()
    })))
}

#[get("/website/config")]
async fn config(db: web::Data<Vec<Database>>) -> Result<HttpResponse, Error> {
    let ret = db::website::get_config("", &db[1]).await;

    if ret.is_err() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        })));
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": 1i64,
        "msg": ret.unwrap()
    })))
}

#[post("/website")]
#[has_permissions("4")]
async fn news_admin(
    req: web::Json<Value>,
    db: web::Data<Vec<Database>>,
) -> Result<HttpResponse, Error> {
    let id;

    if req["id"].as_str().unwrap() == "" {
        id = oid::ObjectId::new();
    } else {
        id = oid::ObjectId::parse_str(req["id"].as_str().unwrap()).unwrap();
    }

    match req["method"].as_str().unwrap() {
        "news" => {
            let ret = db::website::set_news(
                db::website::NewsInfo {
                    id: id,
                    title: req["title"].as_str().unwrap().to_string(),
                    date: req["date"].as_str().unwrap().to_string(),
                    typ: req["type"].as_str().unwrap().to_string(),
                    content: req["content"].as_str().unwrap().to_string(),
                    delete: req["delete"].as_bool().unwrap(),
                },
                &db[1],
            )
            .await;

            if ret.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": ret.unwrap()
            })))
        }
        "carousel" => {
            let ret = db::website::set_carousel(
                db::website::CarouselInfo {
                    id: id,
                    img: req["img"].as_str().unwrap().to_string(),
                    url: req["url"].as_str().unwrap().to_string(),
                    delete: req["delete"].as_bool().unwrap(),
                },
                &db[1],
            )
            .await;

            if ret.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": ret.unwrap()
            })))
        }
        "downloads" => {
            let ret = db::website::set_downloads(
                db::website::DownloadsInfo {
                    id: id,
                    title: req["title"].as_str().unwrap().to_string(),
                    desc: req["desc"].as_str().unwrap().to_string(),
                    url: req["url"].as_str().unwrap().to_string(),
                    delete: req["delete"].as_bool().unwrap(),
                },
                &db[1],
            )
            .await;

            if ret.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": ret.unwrap()
            })))
        }
        "shop" => {
            let ret = db::website::set_shop(
                db::website::ShopInfo {
                    id: id,
                    name: req["name"].as_str().unwrap().to_string(),
                    preview: req["preview"].as_str().unwrap().to_string(),
                    desc: req["desc"].as_str().unwrap().to_string(),
                    cash: req["cash"].as_f64().unwrap(),
                    typ: req["type"].as_str().unwrap().to_string(),
                    delete: req["delete"].as_bool().unwrap(),
                },
                &db[1],
            )
            .await;

            if ret.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": ret.unwrap()
            })))
        }
        "config" => {
            let ret = db::website::set_config(
                db::website::ConfigInfo {
                    id: id,
                    name: req["name"].as_str().unwrap().to_string(),
                    login_server: req["login_server"].as_str().unwrap().to_string(),
                    delete: req["delete"].as_bool().unwrap(),
                },
                &db[1],
            )
            .await;

            if ret.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": ret.unwrap()
            })))
        }
        _ => Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        }))),
    }
}
