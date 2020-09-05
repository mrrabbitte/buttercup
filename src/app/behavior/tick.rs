use actix::{Message, Actor};
use actix::dev::{MessageResponse, ResponseChannel};
use crate::app::agents::core::Agent;

pub struct Tick;

impl Message for Tick {
    type Result = TickStatus;
}

pub enum TickStatus {

    Success,
    Failure,
    Running

}

impl MessageResponse<Agent, Tick> for TickStatus {

    fn handle<R: ResponseChannel<Tick>>(self,
                                        ctx: &mut <Agent as Actor>::Context,
                                        tx: Option<R>) {

    }
}