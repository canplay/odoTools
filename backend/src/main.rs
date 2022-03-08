use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{middleware, middleware::Logger, web::Data, App, HttpServer};
use anyhow::Error;
use chrono::Local;
use log::info;
use log4rs;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

mod db;
mod route;

#[derive(Serialize, Deserialize, Clone)]
pub struct Conf {
    pub url: String,
    pub pg_host: String,
    pub pg_name: String,
    pub pg_user: String,
    pub pg_pwd: String,
    pub mysql_host: String,
    pub mysql_name: String,
    pub mysql_user: String,
    pub mysql_pwd: String,
    pub mongodb_host: String,
    pub mongodb_name: String,
    pub mongodb_user: String,
    pub mongodb_pwd: String,
    pub refresh_rate: u64,
    pub register: bool,
    pub thread_num: usize,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("========== 程序启动 ==========");

    let path = Path::new("config.json");
    let mut f = File::open(path)?;
    let mut s = "".to_string();
    f.read_to_string(&mut s)?;
    let config: Conf = serde_json::from_str(&s)?;

    let client_options;
    if config.mongodb_user != "" && config.mongodb_pwd != "" {
        client_options = ClientOptions::parse(format!(
            "mongodb://{}:{}@{}",
            config.mongodb_user, config.mongodb_pwd, config.mongodb_host
        ))
        .await?;
    } else {
        client_options = ClientOptions::parse(format!("mongodb://{}", config.mongodb_host)).await?;
    }
    let client = Client::with_options(client_options)?;
    let db = client.database(format!("{}", config.mongodb_name).as_str());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::DefaultHeaders::new().add(("Server", "CaNplay")))
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("date", Local::now().format("%Y-%m-%d %H:%M:%S").to_string())),
            )
            .wrap(middleware::Compress::default())
            .wrap(
                CookieSession::signed(&[0; 32])
                    .name("canplay")
                    .secure(false)
                    .http_only(false),
            )
            .service(route::user::register)
            .service(route::user::login)
    })
    .workers(config.thread_num)
    .bind(&config.url)?
    .run()
    .await?;

    info!("========== 程序退出 ==========");
    Ok(())
}
