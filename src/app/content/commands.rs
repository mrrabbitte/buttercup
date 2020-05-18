use crate::app::common::addressable::Address;
use crate::app::content::definitions::ContentType;
use crate::app::values::ValuesPayload;
use crate::app::content::responses::ContentCommandResponse;
use crate::app::content::commands::html::HtmlContentCommandsContext;
use crate::app::content::commands::video::VideoContentCommandsContext;

pub mod video;
pub mod html;

pub struct ContentCommandExecutorContexts {

    html_context: HtmlContentCommandsContext,
    video_context: VideoContentCommandsContext

}

pub struct ContentCommandExecutor {

    tenant_id: String,
    content_type: ContentType,
    commands: Vec<ContentCommand>

}

pub enum ContentCommandExecutionError {

    NoCommandsProvided,
    ContentCommandNotFound(ContentCommandAddress),
    CommandIdMismatch(i32, i32)

}

impl ContentCommandExecutor {

    pub fn execute(&self,
                   contexts: ContentCommandExecutorContexts,
                   payload: &ValuesPayload,
                   content_commands: &Vec<ContentCommandAddress>)
                   -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        match content_commands.get(0) {
            None => return Result::Err(ContentCommandExecutionError::NoCommandsProvided),
            Some(first) => self.do_execute(content_commands),
        }
    }

    fn do_execute(&self,
                  contexts: ContentCommandExecutorContexts,
                  payload: &ValuesPayload,
                  content_commands: &Vec<ContentCommandAddress>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError> {
        let commands =
            self.choose(content_commands);
    }

    fn choose(&self, target: &Vec<ContentCommandAddress>)
        -> Result<Vec<&ContentCommand>, ContentCommandExecutionError> {
        let mut ret: Vec<&ContentCommand> = vec![];
        for address in target {
            let index = address.get_index();
            match self.commands.get(*index) {
                None => return Result::Err(
                    ContentCommandExecutionError::ContentCommandNotFound(address.clone())),
                Some(command) => {
                    if !command.matches(address) {
                        return Result::Err(
                            ContentCommandExecutionError::CommandIdMismatch(
                            *command.get_id(), *address.get_id()))
                    }
                    ret.push(command);
                },
            }
        }
        return Result::Ok(ret);
    }

}

pub enum ContentCommand {

    HtmlCommand,
    VideoCommand

}

impl ContentCommandDelegate for ContentCommand {

    fn get_id(&self) -> &i32 {
        unimplemented!()
    }

    fn get_content_type(&self) -> ContentType {
        unimplemented!()
    }

}

pub trait ContentCommandDelegate {

    fn get_id(&self) -> &i32;
    fn get_content_type(&self) -> ContentType;
    fn matches(&self,
               address: &ContentCommandAddress) -> bool {
        address.get_id() == self.get_id()
    }

}

pub trait ContentCommandsContext {

    fn execute(&self,
               payload: &ValuesPayload,
               content_commands: &Vec<ContentCommand>)
        -> Result<ContentCommandResponse, ContentCommandExecutionError>;

}

#[derive(Debug, Clone, PartialEq, Eq)]
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