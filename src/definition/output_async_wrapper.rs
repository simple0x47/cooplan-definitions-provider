use std::time::Duration;

use async_recursion::async_recursion;
use cooplan_definitions_lib::definition::Definition;
use tokio::time::sleep;

use crate::config::output_config::OutputConfig;

use super::rabbitmq_output::RabbitMQOutput;

pub struct OutputAsyncWrapper {
    config: OutputConfig,
    output: RabbitMQOutput,

    connect_retry_count: i32,
    set_retry_count: i32,
}

impl OutputAsyncWrapper {
    pub fn new(config: OutputConfig, output: RabbitMQOutput) -> OutputAsyncWrapper {
        OutputAsyncWrapper {
            config,
            output,

            connect_retry_count: 0,
            set_retry_count: 0,
        }
    }

    #[async_recursion]
    pub async fn try_connect(&mut self) {
        match self.output.connect().await {
            Ok(_) => {
                self.connect_retry_count = 0;
                log::info!("sucessfully connected to output");
            }
            Err(error) => {
                if self.connect_retry_count >= self.config.connection_retry_count {
                    log::error!("failed to connect to output: {}", error);
                    std::process::exit(1);
                }

                sleep(Duration::from_secs(
                    self.config.connection_retry_interval_seconds,
                ))
                .await;

                self.connect_retry_count += 1;
                log::warn!(
                    "retrying to connect to output, count: {}",
                    self.connect_retry_count
                );

                self.try_connect().await;
            }
        }
    }

    #[async_recursion]
    pub async fn try_set(&mut self, definition: Definition) {
        match self.output.set(&definition).await {
            Ok(_) => {
                self.set_retry_count = 0;
                log::info!("sucessfully set new definition on output");
            }
            Err(error) => {
                if self.set_retry_count >= self.config.set_retry_count {
                    log::error!("failed to set definition on output: {}", error);
                    std::process::exit(1);
                }

                sleep(Duration::from_secs(self.config.set_retry_interval_seconds)).await;

                self.set_retry_count += 1;
                log::warn!(
                    "retrying to set definition on output, count: {}",
                    self.set_retry_count
                );

                self.try_set(definition).await;
            }
        }
    }
}
