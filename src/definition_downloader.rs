use crate::error::Error;

/// This trait provides us with means to make locally available the definitions from wherever they
/// may be stored.
pub trait DefinitionDownloader {
    fn download(&self) -> Result<(), Error>;
    fn update(&self) -> Result<(), Error>;
}
