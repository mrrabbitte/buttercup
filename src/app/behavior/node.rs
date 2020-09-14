use serde::{Deserialize, Serialize};

use crate::app::address::Address;
use buttercup_macros::Address;

pub mod root;
pub mod leaf;
pub mod decorator;
pub mod composite;


#[derive(Address, Serialize, Deserialize, Eq, Hash, PartialEq, PartialOrd, Debug, Clone)]
pub struct BTNodeAddress {

    id: i32,
    index: usize

}
