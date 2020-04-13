use crate::app::common::addressable::Address;

pub mod video;

#[derive(Debug, Clone)]
pub enum ContentType {

    EmailHtml,
    Json,
    Video,
    Image,
    Pdf

}

#[derive(Debug, Clone)]
pub struct ContentCommandDefinition {

    id: i32,
    content_type: ContentType

}

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
