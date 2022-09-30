use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub amqp_channel_name: String,

    pub connection_retry_count: i32,
    pub connection_retry_interval_seconds: u64,

    pub set_retry_count: i32,
    pub set_retry_interval_seconds: u64,
}
