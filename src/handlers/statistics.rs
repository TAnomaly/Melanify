use crate::services::statistics_service::StatisticsService;
use actix_web::{web, HttpResponse};

pub async fn get_user_statistics(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    match StatisticsService::get_user_statistics(&user_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_listening_history() -> HttpResponse {
    match StatisticsService::get_listening_history().await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_daily_stats() -> HttpResponse {
    match StatisticsService::get_daily_stats().await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
