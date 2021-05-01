use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::mapref::one::Ref;

use crate::node::BTNode;
use crate::node::root::RootBTNodeDefinition;
use crate::tree::{BehaviorTree, BehaviorTreeService};

pub struct BehaviorTreeDefinitionService {

    definitions: DashMap<i32, BehaviorTreeDefinition>

}

impl BehaviorTreeDefinitionService {

    pub fn get_definition(&self,
                          id: &i32) -> Option<Ref<i32, BehaviorTreeDefinition>> {
        self.definitions.get(id)
    }

}

pub struct BehaviorTreeDefinition {

    id: i32,
    definitions: Vec<Arc<dyn BehaviorTreeNodeDefinition>>,
    root_node: Box<dyn RootBTNodeDefinition>

}

impl BehaviorTreeDefinition {

    pub fn build(&self,
                 context: &BehaviorTreeBuildingContext) -> Result<BehaviorTree, BehaviorTreeBuildingError> {
        Result::Ok(BehaviorTree::new(self.id, self.root_node.build(&context)?))
    }

    pub fn get_definitions(&self) -> &Vec<Arc<dyn BehaviorTreeNodeDefinition>> {
        &self.definitions
    }

    pub fn get_subtree_ids(&self,
                           service: &BehaviorTreeDefinitionService)
        -> Result<HashSet<i32>, BehaviorTreeBuildingError> {
        let mut ids = HashSet::new();

        for node_definition in self.get_definitions() {
            let subtree_ids = node_definition.get_subtree_ids(service)?;

            if !subtree_ids.is_empty() {
                ids.extend(&subtree_ids);
            }
        }

        Result::Ok(ids)
    }
}


pub trait BehaviorTreeNodeDefinition {

    fn build(&self,
             context: &BehaviorTreeBuildingContext) -> Result<BTNode, BehaviorTreeBuildingError>;

    fn get_id(&self) -> &i32;

    fn get_subtree_ids(&self,
                       _: &BehaviorTreeDefinitionService)
        -> Result<HashSet<i32>, BehaviorTreeBuildingError> {
        Result::Ok(HashSet::new())
    }

}


pub struct BehaviorTreeBuildingService {

    behavior_tree_service: Arc<BehaviorTreeService>,
    definition_service: Arc<BehaviorTreeDefinitionService>

}

impl BehaviorTreeBuildingService {

    pub fn build(&self,
                 id: &i32) -> Result<BehaviorTree, BehaviorTreeBuildingError> {
        match self.definition_service.get_definition(&id) {
            None => Result::Err(BehaviorTreeBuildingError::CouldNotFindSubtreeWithId(*id)),
            Some(definition) => {
                let context =
                    self.get_context(definition.value())?;

                definition.build(&context)
            }
        }
    }

    fn get_context(&self,
                   tree_definition: &BehaviorTreeDefinition)
        -> Result<BehaviorTreeBuildingContext, BehaviorTreeBuildingError> {
        let subtree_ids = tree_definition.get_subtree_ids(&self.definition_service)?;

        let mut subtrees = HashMap::new();

        for subtree_id in subtree_ids {
            let subtree = match self.behavior_tree_service.get_by_id(&subtree_id) {
                None => {
                    let subtree = Arc::new(self.build(&subtree_id)?);
                    self.behavior_tree_service.insert_arc(subtree.clone());
                    subtree
                },

                Some(subtree) => subtree
            };

            subtrees.insert(*subtree.get_id(), subtree);
        }

        Result::Ok(
            BehaviorTreeBuildingContext::new(
                tree_definition.get_definitions()
                    .into_iter()
                    .map(|def| (*def.get_id(), def.clone()))
                    .collect(),
                subtrees))
    }

}

pub enum BehaviorTreeBuildingError {

    CouldNotFindChildDefinitionWithId(i32),
    CouldNotFindTreeWithId(i32),
    CouldNotFindSubtreeWithId(i32),
    ProvidedTreeCannotBeASubtreeError

}


pub struct BehaviorTreeBuildingContext {

    node_definitions: HashMap<i32, Arc<dyn BehaviorTreeNodeDefinition>>,
    subtrees: HashMap<i32, Arc<BehaviorTree>>

}

impl BehaviorTreeBuildingContext {

    pub fn new(node_definitions: HashMap<i32, Arc<dyn BehaviorTreeNodeDefinition>>,
               subtrees: HashMap<i32, Arc<BehaviorTree>>) -> BehaviorTreeBuildingContext {
        BehaviorTreeBuildingContext {
            node_definitions,
            subtrees
        }
    }

    pub fn build_child(&self, id: &i32) -> Result<BTNode, BehaviorTreeBuildingError> {
        match self.node_definitions.get(id) {
            None =>
                Result::Err(BehaviorTreeBuildingError::CouldNotFindChildDefinitionWithId(*id)),
            Some(child_definition) =>
                Result::Ok(child_definition.build(&self)?)
        }
    }

    pub fn build_children(&self, ids: &Vec<i32>) -> Result<Vec<BTNode>, BehaviorTreeBuildingError> {
        let mut ret = Vec::new();

        for id in ids {
            ret.push(self.build_child(id)?);
        }

        Result::Ok(ret)
    }

    pub fn get_subtree(&self,
                       id: &i32) -> Result<Arc<BehaviorTree>, BehaviorTreeBuildingError> {
        match self.subtrees.get(id) {
            None => Result::Err(BehaviorTreeBuildingError::CouldNotFindSubtreeWithId(*id)),
            Some(tree) => Result::Ok(tree.clone())
        }
    }

}