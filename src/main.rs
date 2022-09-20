pub mod config;
pub mod definition_downloader_async_wrapper;
pub mod definition_git_downloader;
pub mod definition_repository;
pub mod error;
pub mod git;

use std::io::{Error, ErrorKind};

use definition_downloader_async_wrapper::DefinitionDownloaderAsyncWrapper;
use definition_git_downloader::DefinitionGitDownloader;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Error> {
    match simple_logger::SimpleLogger::new().env().init() {
        Ok(_) => (),
        Err(error) => {
            return Err(Error::new(
                ErrorKind::Interrupted,
                format!("failed to initialize logger: {}", error),
            ));
        }
    }

    let config = crate::config::config_reader_builder::default().read()?;

    let definition_downloader_config = config.definition_downloader();
    let git_config = config.git();

    let download = task::spawn(async move {
        let definition_git_downloader = DefinitionGitDownloader::new(git_config);
        let mut definition_wrapper = DefinitionDownloaderAsyncWrapper::new(
            definition_git_downloader,
            definition_downloader_config,
        );

        definition_wrapper.run().await;
    });

    download.await;
    Ok(())
}
