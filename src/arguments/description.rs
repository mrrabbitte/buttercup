use crate::arguments::extractors::ValueExtractionPolicy;
use crate::values::ValueType;

pub struct ArgumentDescription {

    id: i32,
    name: String,
    argument_type: ValueType,
    extraction_policy: ValueExtractionPolicy

}



