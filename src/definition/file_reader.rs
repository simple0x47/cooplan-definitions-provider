use cooplan_definitions_io_lib::category_file_io::build_for_all_categories;
use cooplan_definitions_lib::{
    definition::Definition, validated_source_category::ValidatedSourceCategory,
};
use tokio::sync::watch::{Receiver, Sender};

use crate::{definition::downloader_state::DownloaderState, definition::reader_state::ReaderState};

use super::git_version_detector::GitVersionDetector;

/// Retrieves the definitions from a local directory, whenever the downloader downloads or updates that directory.
pub struct FileReader {
    path: String,
    state_sender: Sender<ReaderState>,
    downloader_state_receiver: Receiver<DownloaderState>,
    version_detector: GitVersionDetector,
}

impl FileReader {
    pub fn new(
        path: String,
        state_sender: Sender<ReaderState>,
        downloader_state_receiver: Receiver<DownloaderState>,
        version_detector: GitVersionDetector,
    ) -> FileReader {
        FileReader {
            path,
            state_sender,
            downloader_state_receiver,
            version_detector,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.downloader_state_receiver.changed().await;

            if self.downloader_state_receiver.borrow().available {
                self.read();
            }
        }
    }

    fn read(&self) {
        match build_for_all_categories(self.path.clone()) {
            Ok(categories_io) => {
                let mut categories: Vec<ValidatedSourceCategory> = Vec::new();

                for mut category_io in categories_io {
                    match category_io.read() {
                        Ok(source_category) => {
                            match ValidatedSourceCategory::try_from(source_category) {
                                Ok(category) => categories.push(category),
                                Err(error) => {
                                    log::error!("failed to validate source category: {}", error);

                                    if !self.state_sender.borrow().available {
                                        self.state_sender
                                            .send_replace(ReaderState::new_not_available());
                                    }

                                    return;
                                }
                            }
                        }
                        Err(error) => {
                            log::error!("failed to read category: {}", error);

                            if !self.state_sender.borrow().available {
                                self.state_sender
                                    .send_replace(ReaderState::new_not_available());
                            }

                            return;
                        }
                    }
                }

                match self.version_detector.read_version() {
                    Ok(version) => {
                        log::info!("version detected: {}", version);
                        let definition = Definition::new(version, categories);

                        self.state_sender
                            .send_replace(ReaderState::new(true, definition));
                    }
                    Err(error) => {
                        log::error!("failed to read definition's version: {}", error);
                    }
                }
            }
            Err(error) => {
                log::error!("failed to read category: {}", error);

                if !self.state_sender.borrow().available {
                    self.state_sender
                        .send_replace(ReaderState::new_not_available());
                }
            }
        }
    }
}
