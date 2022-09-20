pub mod definition_downloader;
pub mod definition_git_downloader;
pub mod definition_repository;
pub mod error;
pub mod git;

use std::{io::Error, time::Duration};

use definition_downloader::DefinitionDownloader;
use definition_git_downloader::DefinitionGitDownloader;
use tokio::{
    task,
    time::{self},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let download = task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(3600u64));

        let downloader: DefinitionGitDownloader = DefinitionGitDownloader::new(
            String::from("https://github.com/simple0x47/cooplan-definitions.git"),
            String::from("categories"),
            String::from("origin"),
            String::from("main"),
        );

        match downloader.download() {
            Ok(_) => (),
            Err(error) => {
                println!("failed to download definitions: {}", error);
                std::process::exit(1);
            }
        }

        loop {
            interval.tick().await;

            match downloader.update() {
                Ok(_) => (),
                Err(error) => {
                    println!("failed to update definitions: {}", error);
                }
            }
        }
    });

    download.await;
    Ok(())
}
