use crate::app::values::extractors::{ValueExtractionError, ValueExtractor, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct EmailValueExtractor;

impl ValueExtractor for EmailValueExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        unimplemented!()
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        unimplemented!()
    }

}