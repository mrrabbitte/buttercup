use std::collections::HashSet;
use crate::app::variables::VariableName;

pub trait BTNodeConfiguration {

    fn get_value_names(&self) -> &HashSet<VariableName>;

}