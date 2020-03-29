

pub struct ConditionDefinition {

    id: i32,
    value_name: String

}

pub struct OperatorDefinition {

    id: i32,
    condition_id: i32,
    is_negation: bool,

}