use crate::arguments::extractors::ValueExtractionPolicy;
use crate::arguments::values::ValueType;

pub struct ArgumentDescription {

    id: i32,
    name: String,
    argument_type: ValueType,
    extraction_policy: ValueExtractionPolicy

}



