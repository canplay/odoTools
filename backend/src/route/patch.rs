use actix_web::{post, web, Error, HttpResponse, Result};
use qbsdiff::{Bsdiff, Bspatch};
use serde_json::{json, Value};
use std::{
    fs::File,
    io::{BufReader, Cursor},
    sync::mpsc::channel,
    thread,
};

#[post("/patch")]
async fn api(req: web::Json<Value>) -> Result<HttpResponse, Error> {
    let (sender, receiver) = channel();

    match req["method"].as_str().unwrap() {
        "query" => Ok(HttpResponse::Ok().json(receiver.recv().unwrap())),
        "patch" => {
            thread::spawn(move || {
                let src = File::open(req["source"].as_str().unwrap());
                if src.is_err() {
                    sender
                        .send(json!({
                            "status": 0i64,
                            "msg": "error"
                        }))
                        .unwrap();
                    return;
                }
                let src_buf = BufReader::new(src.unwrap());

                let patch = File::open(req["patch"].as_str().unwrap());
                if patch.is_err() {
                    sender
                        .send(json!({
                            "status": 0i64,
                            "msg": "error"
                        }))
                        .unwrap();
                    return;
                }
                let patch_buf = BufReader::new(patch.unwrap());

                let patcher = Bspatch::new(patch_buf.buffer()).unwrap();
                let mut target = Vec::new();
                patcher
                    .apply(src_buf.buffer(), Cursor::new(&mut target))
                    .unwrap();

                sender
                    .send(json!({
                        "status": 1i64,
                        "msg": "ok"
                    }))
                    .unwrap();
            });

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": "ok"
            })))
        }
        "diff" => {
            thread::spawn(move || {
                let src = File::open(req["source"].as_str().unwrap());
                if src.is_err() {
                    sender
                        .send(json!({
                            "status": 0i64,
                            "msg": "error"
                        }))
                        .unwrap();
                    return;
                }
                let src_buf = BufReader::new(src.unwrap());

                let dist = File::open(req["target"].as_str().unwrap());
                if dist.is_err() {
                    sender
                        .send(json!({
                            "status": 0i64,
                            "msg": "error"
                        }))
                        .unwrap();
                    return;
                }
                let dist_buf = BufReader::new(dist.unwrap());

                let mut patch = Vec::new();
                Bsdiff::new(src_buf.buffer(), dist_buf.buffer())
                    .compare(Cursor::new(&mut patch))
                    .unwrap();

                sender
                    .send(json!({
                        "status": 1i64,
                        "msg": "ok"
                    }))
                    .unwrap();
            });

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": "ok"
            })))
        }
        _ => Ok(HttpResponse::Ok().json(json!({
            "status": 0i64,
            "msg": "error"
        }))),
    }
}
