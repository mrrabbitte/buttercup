use crate::app::common::addressable::Address;
use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::dictionary::{DictionarySelectionError, DictionarySelectionNode};
use crate::app::selection::nodes::recommendation::RecommendationSelectionNode;
use crate::app::selection::nodes::simple::SimpleSelectionNode;
use crate::app::values::ValuesPayload;

pub mod simple;
pub mod dictionary;
pub mod recommendation;

pub trait SelectionNodeDelegate {

    fn get_id(&self) -> &i32;

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress>;

    fn select_content_command_id(&self, payload: &ValuesPayload)
        -> Result<&i32, SelectionNodeError>;

    fn matches(&self, address: &SelectionNodeAddress) -> bool {
        address.get_id() == self.get_id()
    }

}

#[derive(Debug)]
pub struct SelectionNodeDefinition {

    id: i32,
    name: String

}

impl SelectionNodeDefinition {

    pub fn new(id: i32, name: String) -> SelectionNodeDefinition {
        SelectionNodeDefinition {
            id,
            name
        }
    }

}

#[derive(Debug)]
pub enum SelectionNode {

    Simple(SimpleSelectionNode),
    Dictionary(DictionarySelectionNode),
    Recommendation(RecommendationSelectionNode)

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionNodeError {

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionNodeAddress {

    id: i32,
    index: usize

}

impl SelectionNodeDelegate for SelectionNode {

    fn get_id(&self) -> &i32 {
        self.get_delegate().get_id()
    }

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress> {
        self.get_delegate().get_outgoing_edges()
    }

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload) -> Result<&i32, SelectionNodeError> {
        self.get_delegate().select_content_command_id(payload)
    }

}

impl Address for SelectionNodeAddress {

    fn new(id: i32, index: usize) -> Self {
        SelectionNodeAddress{
            id,
            index
        }
    }

    fn get_id(&self) -> &i32 {
        &self.id
    }

    fn get_index(&self) -> &usize {
        &self.index
    }

}
