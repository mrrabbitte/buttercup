use crate::transformations::di::astro::IsDay;
use crate::transformations::di::DiInputTransformation;
use crate::transformations::transformer::{DoubleInputTransformationDefinition, TransformationDefinition, TransformationError, TransformationRequest, TransformationType, Transformer, SingleInputTransformationDefinition};
use crate::values::ValuesPayload;
use crate::transformations::mono::MonoInputTransformation;

pub mod transformer;
pub mod mono;
pub mod di;

pub struct TransformationService;

impl TransformationService {

    pub fn transform(payload: &ValuesPayload)
                     -> Result<ValuesPayload, TransformationError> {
        let mut requests: Vec<TransformationRequest> = Vec::new();
        requests.push(TransformationRequest::new_mono(
            TransformationDefinition::new(1,
                                          TransformationType::SingleInput,
                                          String::from("foundTz")),
            SingleInputTransformationDefinition::new(
                1,
                String::from("geoArg"),
                MonoInputTransformation::FindTimeZoneFromGeoCoordinates)
        ));
        Transformer::transform(payload, &requests)
    }

}


