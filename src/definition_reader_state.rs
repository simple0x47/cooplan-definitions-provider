use std::time::Instant;

use cooplan_definitions_lib::validated_source_category::ValidatedSourceCategory;

#[derive(Debug, Clone)]
pub struct DefinitionReaderState {
    pub available: bool,
    pub categories: Vec<ValidatedSourceCategory>,
    pub last_updated: Instant,
}

impl DefinitionReaderState {
    pub fn new(available: bool, categories: Vec<ValidatedSourceCategory>) -> DefinitionReaderState {
        DefinitionReaderState {
            available: available,
            categories,
            last_updated: Instant::now(),
        }
    }

    pub fn new_not_available() -> DefinitionReaderState {
        DefinitionReaderState {
            available: false,
            categories: Vec::new(),
            last_updated: Instant::now(),
        }
    }

    pub fn categories(&self) -> Vec<ValidatedSourceCategory> {
        self.categories.clone()
    }
}
