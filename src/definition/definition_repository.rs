use cooplan_definitions_lib::validated_source_category::ValidatedSourceCategory;

use crate::error::Error;

pub trait DefinitionRepository {
    fn read_all(&self) -> Result<Vec<ValidatedSourceCategory>, Error>;
}
