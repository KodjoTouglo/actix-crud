use actix_web::{get, web, Responder};
use log;
#[allow(unused)]
use services::{services::health_check::IHealthCheckService, session::session::BackendSession};
#[allow(unused)]
use shaku::HasComponent;

#[get("/health-check")]
pub async fn health_check(session: web::Data<BackendSession>) -> impl Responder {
    log::info!("Running health check");
    session
        .services()
        .health_check_service()
        .health_check()
        .await
}
