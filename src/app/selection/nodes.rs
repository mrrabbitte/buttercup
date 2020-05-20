use crate::app::common::addressable::Address;
use crate::app::content::commands::ContentCommandAddress;
use crate::app::selection::edges::SelectionEdgeAddress;
use crate::app::selection::nodes::context::SelectionNodesContext;
use crate::app::selection::nodes::dictionary::{DictionarySelectionError, DictionarySelectionNode};
use crate::app::selection::nodes::recommendation::{RecommendationSelectionNode, RecommendationSelectionError};
use crate::app::selection::nodes::simple::SimpleSelectionNode;
use crate::app::values::ValuesPayload;

use serde::{Serialize, Deserialize};

pub mod simple;
pub mod dictionary;
pub mod recommendation;
pub mod context;

pub trait SelectionNodeDelegate {

    fn get_id(&self) -> &i32;

    fn get_outgoing_edges(&self) -> &Vec<SelectionEdgeAddress>;

    fn select_content_command_id(&self,
                                 payload: &ValuesPayload,
                                 context: &dyn SelectionNodesContext)
        -> Result<&ContentCommandAddress, SelectionNodeError>;

    fn matches(&self, address: &SelectionNodeAddress) -> bool {
        address.get_id() == self.get_id()
    }

}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum SelectionNode {

    Simple(SimpleSelectionNode),
    Dictionary(DictionarySelectionNode),
    Recommendation(RecommendationSelectionNode)

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionNodeError {

    RecommendationSelectionError(RecommendationSelectionError),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
                                 payload: &ValuesPayload,
                                 context: &dyn SelectionNodesContext)
        -> Result<&ContentCommandAddress, SelectionNodeError> {
        self.get_delegate().select_content_command_id(payload, context)
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
