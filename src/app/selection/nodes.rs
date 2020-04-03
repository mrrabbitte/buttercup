use crate::app::content::ContentCommandId;
use crate::app::values::ValuesPayload;
use crate::app::selection::nodes::simple::SimpleSelectionNode;
use crate::app::selection::nodes::recommendation::RecommendationSelectionNode;
use crate::app::selection::nodes::dictionary::{DictionarySelectionError, DictionarySelectionNode};

pub mod simple;
pub mod dictionary;
pub mod recommendation;

pub trait SelectionNodeDelegate {

    fn get_id(&self) -> &i32;
    fn get_outgoing_edge_ids(&self) -> &Vec<i32>;
    fn select_content_command_id(&self, payload: &ValuesPayload) -> Result<&i32, SelectionError>;

}

pub struct SelectionNodeDefinition {

    id: i32,
    name: String

}

pub enum SelectionNode<'a> {

    Simple(SimpleSelectionNode),
    Dictionary(DictionarySelectionNode),
    Recommendation(RecommendationSelectionNode<'a>)

}

pub enum SelectionError {

    SimpleSelectionError,
    RecommendationSelectionError,
    DictionarySelectionError(DictionarySelectionError)

}

impl SelectionNode {

    fn get_delegate(&self) -> &dyn SelectionNodeDelegate {
        return match self {
            SelectionNode::Simple(node) => node,
            SelectionNode::Dictionary(node) => node,
            SelectionNode::Recommendation(node) => node,
        }
    }

}

impl SelectionNodeDelegate for SelectionNode {

    fn get_id(&self) -> &i32 {
        self.get_delegate().get_id()
    }

    fn get_outgoing_edge_ids(&self) -> &Vec<i32> {
        self.get_delegate().get_outgoing_edge_ids()
    }

    fn select_content_command_id(&self, payload: &ValuesPayload) -> Result<&i32, SelectionError> {
        self.get_delegate().select_content_command_id(payload)
    }

}




