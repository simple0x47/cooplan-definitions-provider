use cooplan_definitions_lib::category::Category;

use crate::error::Error;

pub trait DefinitionRepository {
    fn read_all(&self) -> Result<Vec<Category>, Error>;
}
