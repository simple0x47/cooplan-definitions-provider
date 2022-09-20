use std::time::Duration;

use async_recursion::async_recursion;
use tokio::time;

use crate::{
    config::definition_downloader_config::DefinitionDownloaderConfig,
    definition_git_downloader::DefinitionGitDownloader,
};

pub struct DefinitionDownloaderAsyncWrapper {
    definition_downloader: DefinitionGitDownloader,
    definition_downloader_config: DefinitionDownloaderConfig,

    download_retry_count: i32,
    update_retry_count: i32,
}

impl DefinitionDownloaderAsyncWrapper {
    pub fn new(
        definition_downloader: DefinitionGitDownloader,
        definition_downloader_config: DefinitionDownloaderConfig,
    ) -> DefinitionDownloaderAsyncWrapper {
        DefinitionDownloaderAsyncWrapper {
            definition_downloader,
            definition_downloader_config,
            download_retry_count: 0,
            update_retry_count: 0,
        }
    }

    pub async fn run(&mut self) {
        self.try_download().await;

        let mut interval = time::interval(Duration::from_secs(
            self.definition_downloader_config.update_interval_seconds,
        ));

        loop {
            interval.tick().await;

            self.try_update().await;
        }
    }

    #[async_recursion]
    async fn try_download(&mut self) {
        match self.definition_downloader.download() {
            Ok(_) => {
                log::info!("successfully downloaded definitions");
                self.download_retry_count = 0;
            }
            Err(error) => {
                if self.download_retry_count
                    >= self.definition_downloader_config.download_retry_count
                {
                    log::error!("failed to download definitions: {}", error);
                    std::process::exit(1);
                }

                let mut interval = time::interval(Duration::from_secs(
                    self.definition_downloader_config
                        .download_retry_interval_seconds,
                ));

                interval.tick().await;
                self.download_retry_count += 1;
                log::warn!(
                    "retrying to download definitions, count: {}",
                    self.download_retry_count
                );

                self.try_download().await;
            }
        }
    }

    #[async_recursion]
    async fn try_update(&mut self) {
        match self.definition_downloader.update() {
            Ok(_) => {
                log::info!("sucessfully updated definitions");
                self.update_retry_count = 0;
            }
            Err(error) => {
                if self.update_retry_count >= self.definition_downloader_config.update_retry_count {
                    log::warn!("failed to update definitions: {}", error);
                    return;
                }

                let mut interval = time::interval(Duration::from_secs(
                    self.definition_downloader_config
                        .update_retry_interval_seconds,
                ));

                interval.tick().await;
                self.update_retry_count += 1;
                log::info!(
                    "retrying to update definitions, count: {}",
                    self.update_retry_count
                );

                self.try_update().await;
            }
        }
    }
}
