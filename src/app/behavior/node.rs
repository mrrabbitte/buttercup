pub mod root;
pub mod leaf;
pub mod decorator;
pub mod composite;

pub struct BTNodeAddress {

    id: i32,
    index: usize

}

impl BTNodeAddress {

    pub fn new(id: i32,
               index: usize) -> BTNodeAddress {
        BTNodeAddress {
            id,
            index
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_index(&self) -> &usize {
        &self.index
    }

}

pub enum BTNode {


}