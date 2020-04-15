use crate::app::common::addressable::Address;

pub mod video;
pub mod commands;

#[derive(Debug, Clone)]
pub enum ContentType {

    EmailHtml,
    Json,
    Video,
    Image,
    Pdf

}


