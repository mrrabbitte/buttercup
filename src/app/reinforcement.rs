use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct ReinforcementEvent {

    id: Uuid,
    created_at_utc: NaiveDateTime,
    decision_id: String,
    event_type: ReinforcementEventType

}

#[derive(Debug)]
pub enum ReinforcementEventType {

    Success,
    Failure

}

pub enum ReinforcementEventHandlingError {

    DecisionOfProvidedIdNotFound

}

pub struct ReinforcementService;

impl ReinforcementService {

    pub fn handle(event: ReinforcementEventType) -> Result<(), ReinforcementEventHandlingError> {
        unimplemented!()
    }

    

}