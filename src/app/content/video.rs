use serde_json::Value;

pub struct VideoRequest {

    tree_id: u32,
    payload: Value

}

impl VideoRequest {

    pub fn new(tree_id: u32, payload: Value) -> VideoRequest {
        VideoRequest {
            tree_id,
            payload
        }
    }

}