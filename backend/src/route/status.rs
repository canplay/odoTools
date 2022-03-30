use crate::Conf;
use actix_web::{get, web, Error, HttpResponse};
use actix_web_grants::proc_macro::has_permissions;
use fastping_rs::{
    PingResult::{Idle, Receive},
    Pinger,
};
use ini::Ini;
use serde_json::json;
use std::{fs::File, io::Read, mem, path::Path};
use windows::Win32::{Foundation, System::Diagnostics::ToolHelp};

fn check_process(exe: &str) -> bool {
    let h_snapshot = unsafe { ToolHelp::CreateToolhelp32Snapshot(ToolHelp::TH32CS_SNAPPROCESS, 0) };

    if h_snapshot != Foundation::INVALID_HANDLE_VALUE {
        let mut process: ToolHelp::PROCESSENTRY32 = ToolHelp::PROCESSENTRY32::default();
        process.dwSize = mem::size_of::<ToolHelp::PROCESSENTRY32>() as u32;

        let mut ret = unsafe { ToolHelp::Process32First(h_snapshot, &mut process) };

        while ret.as_bool() {
            let mut v = vec![0];
            for ch in process.szExeFile {
                if ch.0 != 0 {
                    v.push(ch.0);
                }
            }

            let s = String::from_utf8(v).unwrap();
            if s.contains(exe) {
                unsafe { Foundation::CloseHandle(h_snapshot) };
                return true;
            }

            ret = unsafe { ToolHelp::Process32Next(h_snapshot, &mut process) };
        }
    }

    return false;
}

fn check_port(port: i64) -> bool {
    let mut ret = false;
    let (pinger, results) = Pinger::new(None, Some(56)).unwrap();
    pinger.add_ipaddr(&format!("127.0.0.1:{}", port));
    pinger.run_pinger();

    for _i in [0..3] {
        match results.recv() {
            Ok(result) => match result {
                Idle { addr: _ } => {
                    ret = false;
                }
                Receive { addr: _, rtt: _ } => {
                    ret = true;
                }
            },
            Err(_) => ret = false,
        }
    }

    return ret;
}

#[get("/info/server")]
#[has_permissions("4")]
async fn server(conf: web::Data<Conf>) -> Result<HttpResponse, Error> {
    let mongodb_host: Vec<_> = conf.mongodb_host[0].split(":").collect();
    let mongodb_port = mongodb_host[1];

    let config = Ini::load_from_file("./loginserver/bin/configs/network.properties").unwrap();
    let section = config.general_section();
    let loginserver_port = section.get("network.port").unwrap().to_string();

    let path = Path::new("./loginserver/bin/configs/login.properties");
    let mut f = File::open(path)?;
    let mut s = "".to_string();
    f.read_to_string(&mut s)?;
    let conf_loginserver = s;

    let config = Ini::load_from_file("./gameserver/bin/configs/network.properties").unwrap();
    let section = config.general_section();
    let gameserver_port = section.get("network.port").unwrap().to_string();

    let path = Path::new("./gameserver/bin/configs/server.properties");
    let mut f = File::open(path)?;
    let mut s = "".to_string();
    f.read_to_string(&mut s)?;
    let conf_gameserver = s;

    Ok(HttpResponse::Ok().json(json!({
        "status": 1,
        "msg": {
            "database": conf.mongodb_host,
            "register": conf.register,
            "thread_num": conf.thread_num,
            "status": {
                "mongodb": check_port(mongodb_port.parse::<i64>().unwrap()),
                "loginserver": check_port(loginserver_port.parse::<i64>().unwrap()),
                "gameserver": check_port(gameserver_port.parse::<i64>().unwrap()),
            },
            "dir": std::env::current_dir().unwrap().display().to_string(),
            "loginserver": conf_loginserver,
            "gameserver": conf_gameserver
        }
    })))
}
