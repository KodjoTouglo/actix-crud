use std::sync::Arc;
use store::providers::interfaces::health_check::IHealthCheckProvider;
use commons::ApiResult;
use async_trait::async_trait;
#[allow(unused)]
use log::log;
use shaku::{Interface, Component};


#[async_trait]
pub trait IHealthCheckService: Interface {
    async fn health_check(&self) -> ApiResult<()>;
}

#[allow(dead_code)]
#[derive(Component)]
#[shaku(interface = IHealthCheckService)]
pub struct HealthCheckService {
    #[shaku(inject)]
    health_check_provider: Arc<dyn IHealthCheckProvider>
}

#[async_trait]
impl IHealthCheckService for HealthCheckService {
    async fn health_check(&self) -> ApiResult<()> {
        let result = self.health_check_provider.health_check().await;
        match result {
            Err(err) => {
                log::error!("Failed to run health check");
                ApiResult::from_error(500, "500", &err)
            }
            _ => ApiResult::Data(()),
        }
    }
}