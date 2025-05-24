pub mod statistics;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(statistics::config));
}
