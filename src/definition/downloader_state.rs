#[derive(Debug)]
pub struct DownloaderState {
    pub available: bool,
}

impl DownloaderState {
    pub fn new(available: bool) -> DownloaderState {
        DownloaderState { available }
    }
}
