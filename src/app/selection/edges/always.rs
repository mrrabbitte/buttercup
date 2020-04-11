use crate::app::selection::edges::{SelectionEdgeDefinition, SelectionEdgeDelegate, SelectionEdgeError};
use crate::app::selection::nodes::SelectionNodeAddress;
use crate::app::values::ValuesPayload;

pub struct AlwaysTrueSelectionEdge {

    definition: SelectionEdgeDefinition,
    next_selection_node: SelectionNodeAddress

}

impl AlwaysTrueSelectionEdge {

    pub fn new(definition: SelectionEdgeDefinition,
               next_selection_node: SelectionNodeAddress) -> AlwaysTrueSelectionEdge {
        AlwaysTrueSelectionEdge {
            definition,
            next_selection_node
        }
    }

}

impl SelectionEdgeDelegate for AlwaysTrueSelectionEdge {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_next_selection_node(&self) -> &SelectionNodeAddress {
        &self.next_selection_node
    }

    fn can_pass(&self, payload: &ValuesPayload) -> Result<bool, SelectionEdgeError> {
        Result::Ok(true)
    }

    fn is_always_true(&self) -> bool {
        true
    }

}