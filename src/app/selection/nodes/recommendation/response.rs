
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommenderResponse {

    chosen_command_index: usize,
    chosen_command_id: i32

}

impl RecommenderResponse {

    pub fn new(chosen_command_index: usize,
               chosen_command_id: i32) -> RecommenderResponse {
        RecommenderResponse {
            chosen_command_index,
            chosen_command_id
        }
    }

    pub fn get_chosen_command_index(&self) -> &usize {
        &self.chosen_command_index
    }

    pub fn get_chosen_command_id(&self) -> &i32 {
        &self.chosen_command_id
    }

}