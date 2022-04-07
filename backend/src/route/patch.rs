use actix_web::{post, web, Error, HttpResponse, Result};
use qbsdiff::{Bsdiff, Bspatch, ParallelScheme};
use serde_json::{json, Value};
use std::{
    fs::File,
    io::{BufReader, Cursor},
};

#[post("/patch")]
async fn patch(req: web::Json<Value>) -> Result<HttpResponse, Error> {
    match req["method"].as_str().unwrap() {
        "patch" => {
            let src = File::open(req["source"].as_str().unwrap());
            if src.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }
            let src_buf = BufReader::new(src.unwrap());

            let patch = File::open(req["patch"].as_str().unwrap());
            if patch.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }
            let patch_buf = BufReader::new(patch.unwrap());

            let patcher = Bspatch::new(patch_buf.buffer())?;
            let mut target = Vec::new();
            patcher.apply(src_buf.buffer(), Cursor::new(&mut target))?;

            Ok(HttpResponse::Ok().json(json!({
                "status": 1i64,
                "msg": "ok"
            })))
        }
        "diff" => {
            let src = File::open(req["source"].as_str().unwrap());
            if src.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }
            let src_buf = BufReader::new(src.unwrap());

            let dist = File::open(req["target"].as_str().unwrap());
            if dist.is_err() {
                return Ok(HttpResponse::Ok().json(json!({
                    "status": 0i64,
                    "msg": "error"
                })));
            }
            let dist_buf = BufReader::new(dist.unwrap());

            let mut patch = Vec::new();
            Bsdiff::new(src_buf.buffer(), dist_buf.buffer()).compare(Cursor::new(&mut patch))?;

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
