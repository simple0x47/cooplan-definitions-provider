use std::time::Duration;

use async_recursion::async_recursion;
use tokio::{sync::watch::Sender, time::sleep};

use crate::{
    config::definition_downloader_config::DefinitionDownloaderConfig,
    definition::downloader_state::DownloaderState, definition::git_downloader::GitDownloader,
};

pub struct DownloaderAsyncWrapper {
    downloader: GitDownloader,
    config: DefinitionDownloaderConfig,
    state_sender: Sender<DownloaderState>,

    download_retry_count: i32,
    update_retry_count: i32,
}

impl DownloaderAsyncWrapper {
    pub fn new(
        definition_downloader: GitDownloader,
        definition_downloader_config: DefinitionDownloaderConfig,
        definition_downloader_state_sender: Sender<DownloaderState>,
    ) -> DownloaderAsyncWrapper {
        DownloaderAsyncWrapper {
            downloader: definition_downloader,
            config: definition_downloader_config,
            state_sender: definition_downloader_state_sender,

            download_retry_count: 0,
            update_retry_count: 0,
        }
    }

    pub async fn run(&mut self) {
        self.try_download().await;

        let update_duration = Duration::from_secs(self.config.update_interval_seconds);

        loop {
            sleep(update_duration).await;

            self.try_update().await;
        }
    }

    #[async_recursion]
    async fn try_download(&mut self) {
        if self.state_sender.borrow().available {
            self.state_sender.send_replace(DownloaderState::new(false));
        }

        match self.downloader.download() {
            Ok(_) => {
                log::info!("successfully downloaded definitions");
                self.download_retry_count = 0;

                self.state_sender.send_replace(DownloaderState::new(true));
            }
            Err(error) => {
                if self.download_retry_count >= self.config.download_retry_count {
                    log::error!("failed to download definitions: {}", error);
                    std::process::exit(1);
                }

                sleep(Duration::from_secs(
                    self.config.download_retry_interval_seconds,
                ))
                .await;

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
            self.state_sender.send_replace(DownloaderState::new(false));
        }

        match self.downloader.update() {
            Ok(_) => {
                log::info!("sucessfully updated definitions");
                self.update_retry_count = 0;

                self.state_sender.send_replace(DownloaderState::new(true));
            }
            Err(error) => {
                if self.update_retry_count >= self.config.update_retry_count {
                    log::warn!("failed to update definitions: {}", error);
                    return;
                }

                sleep(Duration::from_secs(
                    self.config.update_retry_interval_seconds,
                ))
                .await;

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
