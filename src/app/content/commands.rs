use serde::{Deserialize, Serialize};

use crate::app::common::addressable::Address;
use crate::app::content::commands::html::{HtmlContentCommandError, HtmlContentCommandExecutor, HtmlContentCommandsContext};
use crate::app::content::commands::video::{VideoContentCommandsContext, VideoContentCommandsExecutor};
use crate::app::content::definitions::ContentType;
use crate::app::content::responses::ContentCommandResponse;
use crate::app::files::FilesServiceError;
use crate::app::values::ValuesPayload;

pub mod video;
pub mod html;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentCommandExecutionError {

    NoCommandsProvided,
    ContentCommandNotFound(ContentCommandAddress),
    CommandIdMismatch(i32, i32),
    HtmlContentCommandError(HtmlContentCommandError),
    FilesServiceError(FilesServiceError)

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentCommandExecutor {

    HtmlCommandExecutor(HtmlContentCommandExecutor),
    VideoCommandExecutor(VideoContentCommandsExecutor)

}

impl ContentCommandExecutor {

    fn get_delegate(&self) -> &dyn ContentCommandExecutorDelegate {
        match self {
            ContentCommandExecutor::HtmlCommandExecutor(
                executor) => executor,
            ContentCommandExecutor::VideoCommandExecutor(
                executor) => executor,
        }
    }

}

impl ContentCommandExecutorDelegate for ContentCommandExecutor {

    fn do_execute(&self,
                  contexts: &ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  addresses: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        self.get_delegate().do_execute(contexts, payload, addresses)
    }

    fn get_content_type(&self) -> ContentType {
        self.get_delegate().get_content_type()
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