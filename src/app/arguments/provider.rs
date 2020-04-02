use crate::app::values::extractors::ValueExtractionPolicy;
use crate::app::values::ValueType;

pub struct ArgumentDefinitionProviderService {

    mock_definitions: HashMap<String, ArgumentDefinition>

}

impl ArgumentDefinitionProviderService {

    pub fn get_argument_definitions_for(&self,
                                        tenant_id: String,
                                        selection_tree_id: i32)
        -> &HashMap<String, ArgumentDefinition> {
        &self.mock_definitions
    }

}


