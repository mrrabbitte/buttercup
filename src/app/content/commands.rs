use crate::app::common::addressable::Address;
use crate::app::content::ContentType;
use crate::app::values::ValuesPayload;
use crate::app::content::responses::ContentCommandResponse;

#[derive(Debug, Clone)]
pub struct ContentCommandDefinition {

    id: i32,
    content_type: ContentType

}

pub trait ContentCommandDelegate {

    fn get_id(&self) -> &i32;
    fn matches(&self,
               address: &ContentCommandAddress) -> bool {
        address.get_id() == self.get_id()
    }

}

pub trait ContentCommandsContext<T: ContentCommandDelegate> {

    fn execute(&self,
               payload: ValuesPayload,
               content_commands: Vec<T>) -> ContentCommandResponse;


}

#[derive(Debug, Clone)]
pub struct ContentCommandAddress {

    id: i32,
    index: usize

}

impl Address for ContentCommandAddress {

    fn new(id: i32, index: usize) -> Self {
        ContentCommandAddress {
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