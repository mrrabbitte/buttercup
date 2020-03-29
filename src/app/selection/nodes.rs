use crate::app::values::ValuesPayload;

pub trait SelectionNode {

    fn get_id(&self) -> &i32;
    fn get_outgoing_edge_ids() -> &Vec<i32>;
    fn select_content_command_id(&self, payload: &ValuesPayload) -> &i32;

}

pub struct SelectionNodeDefinition {

    id: i32,
    name: String

}

pub struct SimpleSelectionNodeDetails {

    selection_node_definition_id: i32,
    content_command_definition_id: i32

}

pub struct SimpleSelectionNode {

    definition: SelectionNodeDefinition,
    details: SimpleSelectionNodeDetails

}

impl SelectionNode for SimpleSelectionNode {

    fn get_id(&self) -> &i32 {
        &self.definition.id
    }

    fn get_outgoing_edge_ids() -> &Vec<i32> {
        unimplemented!()
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload) -> &i32 {
        unimplemented!()
    }
}

pub enum SelectionNodeType {

    Simple,
    Dictionary,
    Recommendation

}