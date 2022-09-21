use std::{cell::RefCell, rc::Rc, time::Instant};

use cooplan_definitions_lib::category::Category;

#[derive(Debug, Clone)]
pub struct DefinitionReaderState {
    pub available: bool,
    pub root: Vec<Rc<RefCell<Category>>>,
    pub last_updated: Instant,
}

impl DefinitionReaderState {
    pub fn new(available: bool, root: Vec<Rc<RefCell<Category>>>) -> DefinitionReaderState {
        DefinitionReaderState {
            available: available,
            root: root,
            last_updated: Instant::now(),
        }
    }

    pub fn new_error() -> DefinitionReaderState {
        DefinitionReaderState {
            available: false,
            root: Vec::new(),
            last_updated: Instant::now(),
        }
    }

    pub fn root(&self) -> Vec<Rc<RefCell<Category>>> {
        self.root.clone()
    }
}
