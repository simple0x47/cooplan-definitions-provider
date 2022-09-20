use serde::{Deserialize, Serialize};

use super::{definition_downloader_config::DefinitionDownloaderConfig, git_config::GitConfig};

#[derive(Deserialize, Serialize)]
pub struct Config {
    git: GitConfig,
    definition_downloader: DefinitionDownloaderConfig,
}

impl Config {
    pub fn git(&self) -> GitConfig {
        self.git.clone()
    }

    pub fn definition_downloader(&self) -> DefinitionDownloaderConfig {
        self.definition_downloader.clone()
    }
}
