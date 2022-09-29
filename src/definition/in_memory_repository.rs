use cooplan_definitions_lib::validated_source_category::ValidatedSourceCategory;
use tokio::sync::watch::Receiver;

use crate::{
    definition::reader_state::ReaderState,
    definition::repository::Repository,
    error::{Error, ErrorKind},
};

pub struct InMemoryRepository {
    reader_state_receiver: Receiver<ReaderState>,
}

impl InMemoryRepository {
    pub fn new(reader_state_receiver: Receiver<ReaderState>) -> InMemoryRepository {
        InMemoryRepository {
            reader_state_receiver,
        }
    }
}

impl Repository for InMemoryRepository {
    fn read_all(&self) -> Result<Vec<ValidatedSourceCategory>, Error> {
        if !self.reader_state_receiver.borrow().available {
            return Err(Error::new(
                ErrorKind::DefinitionsNotAvailable,
                "definitions are unavailable",
            ));
        }

        Ok(self.reader_state_receiver.borrow().categories())
    }
}
