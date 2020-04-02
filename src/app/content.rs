pub mod video;


pub enum ContentType {

    EmailHtml,
    Json,
    Video,
    Image,
    Pdf

}

pub struct ContentCommandDefinition {

    id: i32

}

pub struct ContentCommandId {

    value: i32

}

impl ContentCommandId {

    pub fn new(value: &i32) -> ContentCommandId {
        ContentCommandId {
            value: *value
        }
    }

    pub fn get(&self) -> &i32 {
        &self.value
    }

}
