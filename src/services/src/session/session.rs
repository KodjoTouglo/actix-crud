use crate::factory::IBackendServices;
#[allow(unused_imports)]
use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

pub struct BackendSession {
    services: Arc<dyn IBackendServices + Send + Sync>,
}

impl Clone for BackendSession {
    fn clone(&self) -> Self {
        Self {
            services: self.services.clone(),
        }
    }
}

impl BackendSession {
    pub fn new(services: Arc<dyn IBackendServices + Send + Sync>) -> Self {
        Self {
            services: Arc::clone(&services),
        }
    }

    pub fn services(&self) -> &Arc<dyn IBackendServices + Send + Sync> {
        &self.services
    }
}
