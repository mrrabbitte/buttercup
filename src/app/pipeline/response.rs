use chrono::NaiveDateTime;
use crate::app::content::ContentType;
use url::Url;

pub struct ContentPipelineResponse {
s
    id: String,
    created_at_utc: NaiveDateTime,
    content_type: ContentType,
    url: Url

}