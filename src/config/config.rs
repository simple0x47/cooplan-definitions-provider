use cooplan_definition_git_downloader::git_config::GitConfig;
use serde::{Deserialize, Serialize};

use super::{
    definition_downloader_config::DefinitionDownloaderConfig, output_config::OutputConfig,
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    git: GitConfig,
    definition_downloader: DefinitionDownloaderConfig,
    output: OutputConfig,
}

impl Config {
    pub fn git(&self) -> GitConfig {
        self.git.clone()
    }

    pub fn definition_downloader(&self) -> DefinitionDownloaderConfig {
        self.definition_downloader.clone()
    }

    pub fn output(&self) -> OutputConfig {
        self.output.clone()
    }
}
