use crate::Conf;
use actix_web::{post, web, Error, HttpResponse};
use serde_json::{json, Value};
use std::{fs::File, io::Read, path::Path};

#[post("/info/server")]
async fn server(req: web::Json<Value>, conf: web::Data<Conf>) -> Result<HttpResponse, Error> {
    match req["method"].as_str().unwrap() {
        "config" => {
            let path = Path::new("./web/config.json");
            let mut f = File::open(path)?;
            let mut s = "".to_string();
            f.read_to_string(&mut s)?;
            let conf_web: Value = serde_json::from_str(&s)?;

            let path = Path::new("./loginserver/bin/configs/login.properties");
            let mut f = File::open(path)?;
            let mut s = "".to_string();
            f.read_to_string(&mut s)?;
            let conf_loginserver = s;

            let path = Path::new("./gameserver/bin/configs/server.properties");
            let mut f = File::open(path)?;
            let mut s = "".to_string();
            f.read_to_string(&mut s)?;
            let conf_gameserver = s;

            Ok(HttpResponse::Ok().json(json!({
                "status": "1",
                "msg": {
                    "url": conf.url,
                    "database": conf.mongodb_host,
                    "register": conf.register,
                    "thread_num": conf.thread_num,
                    "dir": std::env::current_dir().unwrap().display().to_string(),
                    "config": {
                        "web": conf_web,
                        "loginserver": conf_loginserver,
                        "gameserver": conf_gameserver
                    }
                }
            })))
        }
        _ => Ok(HttpResponse::Ok().json(json!({
            "status": "0",
            "msg": "error"
        }))),
    }
}
