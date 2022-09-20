use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct DefinitionDownloaderConfig {
    pub update_interval_seconds: u64,
    pub download_retry_count: i32,
    pub download_retry_interval_seconds: u64,
    pub update_retry_count: i32,
    pub update_retry_interval_seconds: u64,
}
