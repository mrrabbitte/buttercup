use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct Email {

    value: String

}

impl Email {

    pub fn new(value: String) -> Email {
        Email {
            value
        }
    }

}