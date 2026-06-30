use crate::services::health_check::{HealthCheckService, IHealthCheckService};
use crate::services::user::{IUserService, UserService};
#[allow(unused)]
use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;
use store::providers::{
    client::postgres_client::DataBaseManager,
    repositories::health_check_repo::HealthCheckProvider,
    repositories::user_repo::UserProvider,
};

module! {
    pub BackendServicesCatalog {
        components = [
            DataBaseManager,
            HealthCheckProvider,
            UserProvider,
            UserService,
            HealthCheckService,
        ],
        providers = [],
    }
}

pub trait IBackendServices {
    fn health_check_service(&self) -> Arc<dyn IHealthCheckService>;

    fn user_service(&self) -> Arc<dyn IUserService>;
}

pub struct BackendServicesFactory {
    services: BackendServicesCatalog,
}

impl BackendServicesFactory {
    pub fn new(services: BackendServicesCatalog) -> Self {
        Self { services }
    }
}

impl IBackendServices for BackendServicesFactory {
    fn health_check_service(&self) -> Arc<dyn IHealthCheckService> {
        let health_check: Arc<dyn IHealthCheckService> = self.services.resolve();
        health_check
    }

    fn user_service(&self) -> Arc<dyn IUserService> {
        let user_service: Arc<dyn IUserService> = self.services.resolve();
        user_service
    }
}
