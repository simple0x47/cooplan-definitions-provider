use std::time::Instant;

use cooplan_definitions_lib::validated_source_category::ValidatedSourceCategory;

#[derive(Debug, Clone)]
pub struct ReaderState {
    pub available: bool,
    pub categories: Vec<ValidatedSourceCategory>,
    pub last_updated: Instant,
}

impl ReaderState {
    pub fn new(available: bool, categories: Vec<ValidatedSourceCategory>) -> ReaderState {
        ReaderState {
            available: available,
            categories,
            last_updated: Instant::now(),
        }
    }

    pub fn new_not_available() -> ReaderState {
        ReaderState {
            available: false,
            categories: Vec::new(),
            last_updated: Instant::now(),
        }
    }

    pub fn categories(&self) -> Vec<ValidatedSourceCategory> {
        self.categories.clone()
    }
}
