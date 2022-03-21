use crate::Conf;
use actix_web::{get, web, Error, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use serde_json::{json, Value};
use std::{fs::File, io::Read, path::Path};

#[get("/info/server")]
#[has_permissions("4")]
async fn server(conf: web::Data<Conf>) -> Result<HttpResponse, Error> {
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
