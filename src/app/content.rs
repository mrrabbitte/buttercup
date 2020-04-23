use crate::app::common::addressable::Address;

pub mod commands;
pub mod responses;

#[derive(Debug, Clone)]
pub enum ContentType {

    Html,
    Json,
    Video,
    Image,
    Pdf

}


