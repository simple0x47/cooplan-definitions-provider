use std::{cell::RefCell, rc::Rc};

use cooplan_definitions_io_lib::{
    category_file_io::build_for_all_categories, source_category_converter::SourceCategoryConverter,
};
use cooplan_definitions_lib::category::Category;
use tokio::sync::watch::{Receiver, Sender};

use crate::{
    definition_downloader_state::DefinitionDownloaderState,
    definition_reader_state::DefinitionReaderState,
};

pub struct DefinitionFileReader {
    path: String,
    state_sender: Sender<DefinitionReaderState>,
    downloader_state_receiver: Receiver<DefinitionDownloaderState>,
}

impl DefinitionFileReader {
    pub fn new(
        path: String,
        state_sender: Sender<DefinitionReaderState>,
        downloader_state_receiver: Receiver<DefinitionDownloaderState>,
    ) -> DefinitionFileReader {
        DefinitionFileReader {
            path,
            state_sender,
            downloader_state_receiver,
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
                let mut converter = SourceCategoryConverter::new();
                let mut categories: Vec<Rc<RefCell<Category>>> = Vec::new();

                for mut category_io in categories_io {
                    match category_io.read() {
                        Ok(source_category) => match converter.convert(source_category) {
                            Ok(category) => categories.push(category),
                            Err(error) => {
                                log::error!("failed to convert source category: {}", error);

                                if !self.state_sender.borrow().available {
                                    self.state_sender
                                        .send_replace(DefinitionReaderState::new_error());
                                }

                                return;
                            }
                        },
                        Err(error) => {
                            log::error!("failed to read category: {}", error);

                            if !self.state_sender.borrow().available {
                                self.state_sender
                                    .send_replace(DefinitionReaderState::new_error());
                            }

                            return;
                        }
                    }
                }

                self.state_sender
                    .send_replace(DefinitionReaderState::new(true, categories));
            }
            Err(error) => {
                log::error!("failed to read category: {}", error);

                if !self.state_sender.borrow().available {
                    self.state_sender
                        .send_replace(DefinitionReaderState::new_error());
                }
            }
        }
    }
}
