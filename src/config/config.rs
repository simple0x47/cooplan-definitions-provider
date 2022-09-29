use serde::{Deserialize, Serialize};

use super::{definition_downloader_config::DefinitionDownloaderConfig, git_config::GitConfig};

#[derive(Deserialize, Serialize)]
pub struct Config {
    git: GitConfig,
    definition_downloader: DefinitionDownloaderConfig,
    amqp_channel: String,
}

impl Config {
    pub fn git(&self) -> GitConfig {
        self.git.clone()
    }

    pub fn definition_downloader(&self) -> DefinitionDownloaderConfig {
        self.definition_downloader.clone()
    }

    pub fn amqp_channel(&self) -> String {
        self.amqp_channel.clone()
    }
}
