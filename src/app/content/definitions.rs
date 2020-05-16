use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ContentCommandDefinition {

    id: i32,
    content_type: ContentType

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {

    Html,
    Video,

}
