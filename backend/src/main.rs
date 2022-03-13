use actix_cors::Cors;
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{
    cookie::time::Duration,
    dev::ServiceRequest,
    get, middleware,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use anyhow::Error;
use chrono::Local;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};
use tracing::{info, instrument, Level};
use tracing_actix_web::TracingLogger;
use tracing_appender::rolling;
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{self, format::Writer, time::FormatTime},
    layer::SubscriberExt,
    EnvFilter,
};

mod claims;
mod db;
mod route;

#[derive(Clone, Debug, Deserialize, Serialize)]
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

struct LocalTimer;
impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let claims = claims::decode_jwt(credentials.token())?;
    req.attach(claims.permissions);
    Ok(req)
}

#[get("/")]
async fn index() -> actix_web::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./web/index.html")?)
}

#[actix_web::main]
#[instrument]
async fn main() -> Result<(), Error> {
    LogTracer::init().expect("failed to set logger");
    let file_appender = rolling::daily(".", "log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(Level::ERROR.into())
                .add_directive(Level::WARN.into())
                .add_directive(Level::INFO.into()),
        )
        .with(
            fmt::Layer::new().with_writer(io::stdout).event_format(
                tracing_subscriber::fmt::format()
                    .with_level(true)
                    .with_target(true)
                    .with_timer(LocalTimer),
            ),
        )
        .with(
            fmt::Layer::new()
                .with_writer(non_blocking)
                .event_format(
                    tracing_subscriber::fmt::format()
                        .with_level(true)
                        .with_target(true)
                        .with_ansi(false)
                        .with_timer(LocalTimer),
                )
                .with_ansi(false),
        );
    tracing::subscriber::set_global_default(subscriber).expect("unable to set a global collector");

    info!("========== backend start ==========");

    let path = Path::new("config.json");
    let mut f = File::open(path)?;
    let mut s = "".to_string();
    f.read_to_string(&mut s)?;
    let config: Conf = serde_json::from_str(&s)?;
    let config_data: Conf = serde_json::from_str(&s)?;

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

    info!("========== {} ==========", config.url);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config_data.clone()))
            .app_data(Data::new(db.clone()))
            .wrap(TracingLogger::default())
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
                    .http_only(true)
                    .expires_in_time(Duration::hours(1)),
            )
            .service(index)
            .service(route::user::register)
            .service(route::user::login)
            .service(route::info::server)
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(validator))
                    .service(route::user::info),
            )
            .service(fs::Files::new("/", "./web").show_files_listing())
    })
    .workers(config.thread_num)
    .bind(&config.url)?
    .run()
    .await?;

    info!("========== backend stop ==========");
    Ok(())
}
