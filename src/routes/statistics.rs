use crate::handlers::statistics::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/statistics")
            .route("/user/{user_id}", web::get().to(get_user_statistics))
            .route("/history", web::get().to(get_listening_history))
            .route("/daily", web::get().to(get_daily_stats)),
    );
}
