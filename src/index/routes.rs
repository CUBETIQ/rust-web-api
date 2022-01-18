use crate::index::ApiInfo;
use actix_web::{get, web, HttpResponse, Responder};
use std::time::UNIX_EPOCH;

use nix::unistd;

#[get("/")]
async fn index() -> impl Responder {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf).expect("Failed getting hostname");
    let mut hostname = String::new();
    hostname.push_str(hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8"));

    HttpResponse::Ok().json(ApiInfo {
        instance_id: hostname,
        status: "ok".to_string(),
        timestamp: UNIX_EPOCH.elapsed().unwrap().as_secs(),
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
}
