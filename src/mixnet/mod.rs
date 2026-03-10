pub mod keygen;
pub mod shuffle;
pub mod zk;

use crate::preliminaries::{MixnetResult, SecurityParams};

#[derive(Debug, Clone)]
pub struct MixnetConfig {
    pub params: SecurityParams,
}

impl MixnetConfig {
    pub fn new(params: SecurityParams) -> MixnetResult<Self> {
        params.validate()?;
        Ok(Self { params })
    }
}

#[derive(Debug)]
pub struct Mixnet {
    config: MixnetConfig,
}

impl Mixnet {
    pub fn new(config: MixnetConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &MixnetConfig {
        &self.config
    }
}
