use crate::app::common::addressable::Address;
use crate::app::content::definitions::ContentType;
use crate::app::values::ValuesPayload;
use crate::app::content::responses::ContentCommandResponse;
use crate::app::content::commands::html::{HtmlContentCommandsContext, HtmlContentCommandExecutor, HtmlContentCommandError};
use crate::app::content::commands::video::VideoContentCommandsContext;

pub mod video;
pub mod html;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct ContentCommandExecutorContexts {

    html_context: HtmlContentCommandsContext,
    video_context: VideoContentCommandsContext

}

impl ContentCommandExecutorContexts {

    pub fn new(html_context: HtmlContentCommandsContext,
               video_context: VideoContentCommandsContext) -> ContentCommandExecutorContexts {
        ContentCommandExecutorContexts {
            html_context,
            video_context
        }
    }

    pub fn get_html_context(&self) -> &HtmlContentCommandsContext {
        &self.html_context
    }

    pub fn get_video_context(&self) -> &VideoContentCommandsContext {
        &self.video_context
    }

}

#[derive(Debug, Clone)]
pub enum ContentCommandExecutionError {

    NoCommandsProvided,
    ContentCommandNotFound(ContentCommandAddress),
    CommandIdMismatch(i32, i32),
    HtmlContentCommandError(HtmlContentCommandError)

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentCommandExecutor {

    HtmlCommandExecutor(HtmlContentCommandExecutor),
    VideoCommandExecutor

}

impl ContentCommandExecutorDelegate for ContentCommandExecutor {

    fn do_execute(&self,
                  contexts: &ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        unimplemented!()
    }

    fn get_content_type(&self) -> &ContentType {
        unimplemented!()
    }
}

pub trait ContentCommandExecutorDelegate {

    fn execute(&self,
                   contexts: &ContentCommandExecutorContexts,
                   payload: &ValuesPayload,
                   addresses: &Vec<ContentCommandAddress>)
                   -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        if addresses.is_empty() {
            return Result::Err(ContentCommandExecutionError::NoCommandsProvided);
        }
        self.do_execute(contexts, payload, addresses)
    }

    fn do_execute(&self,
                  contexts: &ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  addresses: &Vec<ContentCommandAddress>)
                  -> Result<ContentCommandResponse, ContentCommandExecutionError>;

    fn get_content_type(&self) -> ContentType;

}

pub trait ContentCommand {

    fn get_id(&self) -> &i32;
    fn matches(&self,
               address: &ContentCommandAddress) -> bool {
        address.get_id() == self.get_id()
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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