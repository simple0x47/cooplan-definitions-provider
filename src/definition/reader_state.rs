use std::time::Instant;

use cooplan_definitions_lib::definition::Definition;

#[derive(Debug, Clone)]
pub struct ReaderState {
    pub available: bool,
    pub definition: Option<Definition>,
    pub last_updated: Instant,
}

impl ReaderState {
    pub fn new(available: bool, definition: Definition) -> ReaderState {
        ReaderState {
            available: available,
            definition: Some(definition),
            last_updated: Instant::now(),
        }
    }

    pub fn new_not_available() -> ReaderState {
        ReaderState {
            available: false,
            definition: None,
            last_updated: Instant::now(),
        }
    }

    pub fn definition(&self) -> Option<Definition> {
        self.definition.clone()
    }
}
