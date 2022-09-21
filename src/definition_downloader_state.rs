#[derive(Debug)]
pub struct DefinitionDownloaderState {
    pub available: bool,
}

impl DefinitionDownloaderState {
    pub fn new(available: bool) -> DefinitionDownloaderState {
        DefinitionDownloaderState { available }
    }
}
