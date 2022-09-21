use std::time::Duration;

use async_recursion::async_recursion;
use tokio::{sync::watch::Sender, time};

use crate::{
    config::definition_downloader_config::DefinitionDownloaderConfig,
    definition_downloader_state::DefinitionDownloaderState,
    definition_git_downloader::DefinitionGitDownloader,
};

pub struct DefinitionDownloaderAsyncWrapper {
    downloader: DefinitionGitDownloader,
    config: DefinitionDownloaderConfig,
    state_sender: Sender<DefinitionDownloaderState>,

    download_retry_count: i32,
    update_retry_count: i32,
}

impl DefinitionDownloaderAsyncWrapper {
    pub fn new(
        definition_downloader: DefinitionGitDownloader,
        definition_downloader_config: DefinitionDownloaderConfig,
        definition_downloader_state_sender: Sender<DefinitionDownloaderState>,
    ) -> DefinitionDownloaderAsyncWrapper {
        DefinitionDownloaderAsyncWrapper {
            downloader: definition_downloader,
            config: definition_downloader_config,
            state_sender: definition_downloader_state_sender,

            download_retry_count: 0,
            update_retry_count: 0,
        }
    }

    pub async fn run(&mut self) {
        self.try_download().await;

        let mut interval = time::interval(Duration::from_secs(self.config.update_interval_seconds));

        loop {
            interval.tick().await;

            self.try_update().await;
        }
    }

    #[async_recursion]
    async fn try_download(&mut self) {
        if self.state_sender.borrow().available {
            self.state_sender
                .send_replace(DefinitionDownloaderState::new(false));
        }

        match self.downloader.download() {
            Ok(_) => {
                log::info!("successfully downloaded definitions");
                self.download_retry_count = 0;

                self.state_sender
                    .send_replace(DefinitionDownloaderState::new(true));
            }
            Err(error) => {
                if self.download_retry_count >= self.config.download_retry_count {
                    log::error!("failed to download definitions: {}", error);
                    std::process::exit(1);
                }

                let mut interval = time::interval(Duration::from_secs(
                    self.config.download_retry_interval_seconds,
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
        if self.state_sender.borrow().available {
            self.state_sender
                .send_replace(DefinitionDownloaderState::new(false));
        }

        match self.downloader.update() {
            Ok(_) => {
                log::info!("sucessfully updated definitions");
                self.update_retry_count = 0;

                self.state_sender
                    .send_replace(DefinitionDownloaderState::new(true));
            }
            Err(error) => {
                if self.update_retry_count >= self.config.update_retry_count {
                    log::warn!("failed to update definitions: {}", error);
                    return;
                }

                let mut interval = time::interval(Duration::from_secs(
                    self.config.update_retry_interval_seconds,
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