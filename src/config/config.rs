use serde::{Deserialize, Serialize};

use super::git_config::GitConfig;

#[derive(Deserialize, Serialize)]
pub struct Config {
    git: GitConfig,
}

impl Config {
    pub fn git(&self) -> GitConfig {
        self.git.clone()
    }
}
