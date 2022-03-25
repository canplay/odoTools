use crate::{claims, db, Conf};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Result};
use actix_web_grants::proc_macro::has_any_permission;
use bcrypt;
use chrono::Local;
use mongodb::Database;
use rand::Rng;
use serde_json::{json, Value};

#[post("/user/register")]
async fn register(
    req: web::Json<Value>,
    http_req: HttpRequest,
    db: web::Data<Vec<Database>>,
    conf: web::Data<Conf>,
) -> Result<HttpResponse, Error> {
    if !conf.register {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0,
            "msg": "closed register".to_string()
        })));
    }

    let result = db::user::get_user(req["username"].as_str().unwrap(), &db[0]).await;
    if result.is_ok() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0,
            "msg": "account already exist".to_string()
        })));
    }

    let hashed =
        bcrypt::hash_with_result(req["password"].as_str().unwrap().to_string(), 10).unwrap();
    let mut rng = rand::thread_rng();

    let result = db::user::add_user(
        db::user::AccountInfo {
            id: Local::now().timestamp_millis() + rng.gen_range(0..100),
            account_name: req["username"].as_str().unwrap().to_string(),
            email: req["username"].as_str().unwrap().to_string(),
            password: hashed.format_for_version(bcrypt::Version::TwoA),
            pin: "3".to_string(),
            family: "".to_string(),
            access_lvl: 1i64,
            character_slots: 0i64,
            cash: 0i64,
            confirmation_hash: "".to_string(),
            change_password_hash: "".to_string(),
            registration_date: Local::now().timestamp_millis(),
            wait_check: false,
            pc_code: None,
            host: http_req
                .connection_info()
                .peer_addr()
                .unwrap_or_default()
                .to_string(),
            game_option: db::user::GameOption {
                game_option_data: None,
            },
            ui_info: db::user::UiInfo { ui_data: None },
            macroses: vec![
                db::user::Macroses {
                    index: 0i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 1i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 2i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 3i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 4i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 5i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 6i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 7i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 8i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
                db::user::Macroses {
                    index: 9i64,
                    typee: 3i64,
                    macros_data: "".to_string(),
                },
            ],
        },
        &db[0],
    )
    .await
    .unwrap();

    if result {
        Ok(HttpResponse::Ok().json(json!({
            "status": 1,
            "msg": result
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "status": 0,
            "msg": "register error"
        })))
    }
}

#[post("/user/login")]
async fn login(req: web::Json<Value>, db: web::Data<Vec<Database>>) -> Result<HttpResponse, Error> {
    let result = db::user::get_user(req["username"].as_str().unwrap(), &db[0]).await;

    if result.is_err() {
        return Ok(HttpResponse::Ok().json(json!({
            "status": 0,
            "msg": "no account"
        })));
    }

    let account = result.unwrap();

    let result = bcrypt::verify(
        req["password"].as_str().unwrap().to_string(),
        &account.password,
    )
    .unwrap();

    if result {
        let claims = claims::Claims::new(
            req["username"].as_str().unwrap().to_string(),
            req["password"].as_str().unwrap().to_string(),
            vec![account.access_lvl.to_string()],
        );
        let jwt = claims::create_jwt(claims)?;

        Ok(HttpResponse::Ok().json(json!({
            "status": 1,
            "msg": "loginin",
            "token": jwt
        })))
    } else {
        Ok(HttpResponse::Ok().json(json!({
            "status": 0,
            "msg": "account or password error"
        })))
    }
}

#[get("/user/{username}")]
#[has_any_permission["1", "2", "3", "4"]]
async fn info(
    path: web::Path<String>,
    db: web::Data<Vec<Database>>,
) -> Result<HttpResponse, Error> {
    let result = db::user::get_user(&path.into_inner(), &db[0])
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "status": 1,
        "msg": {
            "family": result.family,
            "cash": result.cash,
            "access_level": result.account_name
        }
    })))
}
