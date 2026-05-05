use std::sync::Arc;

/**
* ### Config
* Configuración global de lógica de negocio del proyecto.
*/

#[derive(Clone)]
pub struct AppConfig {
    pub governance: Arc<GovernanceConfig>,
}

pub struct GovernanceConfig {
    pub quorum: u8,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            governance: Arc::new(GovernanceConfig { quorum: 2 }),
        }
    }
}
