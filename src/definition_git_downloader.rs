use git2::Repository;

use crate::{
    definition_downloader::DefinitionDownloader,
    error::{Error, ErrorKind},
    git::{clone::git_clone, pull::git_pull},
};

pub struct DefinitionGitDownloader {
    repository_url: String,
    repository_local_dir: String,
    remote_name: String,
    remote_branch: String,
}

impl DefinitionGitDownloader {
    pub fn new(
        repository_url: String,
        repository_local_dir: String,
        remote_name: String,
        remote_branch: String,
    ) -> DefinitionGitDownloader {
        DefinitionGitDownloader {
            repository_url,
            repository_local_dir,
            remote_name,
            remote_branch,
        }
    }

    fn clone_repository(&self) -> Result<Repository, Error> {
        match git_clone(
            self.repository_url.as_str(),
            self.repository_local_dir.as_str(),
            self.remote_branch.as_str(),
        ) {
            Ok(repository) => Ok(repository),
            Err(error) => Err(error),
        }
    }

    fn get_repository(&self) -> Result<Repository, Error> {
        match Repository::open(self.repository_local_dir.as_str()) {
            Ok(repository) => Ok(repository),
            Err(_) => self.clone_repository(),
        }
    }
}

impl DefinitionDownloader for DefinitionGitDownloader {
    fn download(&self) -> Result<(), Error> {
        match self.get_repository() {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn update(&self) -> Result<(), Error> {
        match self.get_repository() {
            Ok(repository) => {
                match git_pull(
                    &repository,
                    self.remote_name.as_str(),
                    self.remote_branch.as_str(),
                ) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(Error::new(
                        ErrorKind::FailedToUpdateDefinitions,
                        format!("failed to update definitions: {}", error).as_str(),
                    )),
                }
            }
            Err(error) => Err(error),
        }
    }
}
