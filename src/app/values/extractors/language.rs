use crate::app::values::extractors::{ValueExtractor, ValueExtractionError, ValueExtractorInput};
use crate::app::values::ValueHolder;

pub struct LanguageValueExtractor;

impl ValueExtractor for LanguageValueExtractor {

    fn strict_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        unimplemented!()
    }

    fn lax_extract(input: &ValueExtractorInput) -> Result<ValueHolder, ValueExtractionError> {
        unimplemented!()
    }

}