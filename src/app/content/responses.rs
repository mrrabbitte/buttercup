use url::Url;
use crate::app::content::ContentType;

pub struct ContentCommandResponse {

    resource_id: i32,
    content_type: ContentType,
    url: Url,

}