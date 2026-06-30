mod configs;
mod metrics;
mod middleware;
mod api;

use std::sync::Arc;
use actix_web;
use actix_web::{web::Data, App, HttpServer};
use configs::EnvironmentConfig;
use deadpool_postgres::{tokio_postgres, Runtime};
use dotenv::dotenv;
use services::factory::{BackendServicesCatalog, BackendServicesFactory};
use services::session::session::BackendSession;
use store::providers::client::postgres_client::{DataBaseManager, DataBaseManagerParameters};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = EnvironmentConfig::static_configs();
    let connection_pool = config
        .database_config()
        .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .unwrap();

    let services_factory = BackendServicesCatalog::builder()
        .with_component_parameters::<DataBaseManager>(DataBaseManagerParameters {
            connection_pool: Some(connection_pool),
        })
        .build();

    let services = Arc::new(BackendServicesFactory::new(services_factory));
    let session = Data::new(BackendSession::new(services));

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
            .app_data(session.clone())
            .configure(api::router_config::register_apis)
    })
        .bind((config.server_host(), config.server_port()))?
        .run()
        .await
}
