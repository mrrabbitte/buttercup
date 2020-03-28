use crate::app::transformations::di::astro::IsDay;
use crate::app::transformations::di::DiInputTransformation;
use crate::app::transformations::transformer::{DoubleInputTransformationDefinition, TransformationDefinition, TransformationError, TransformationRequest, TransformationType, Transformer, SingleInputTransformationDefinition};
use crate::app::values::ValuesPayload;
use crate::app::transformations::mono::MonoInputTransformation;

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
        requests.push(TransformationRequest::new_di(
            TransformationDefinition::new(2,
                                          TransformationType::DoubleInput,
                                          String::from("zonedDateTimeValue")),
            DoubleInputTransformationDefinition::new(
                2,
                String::from("dateTimeArg"),
                String::from("foundTz"),
                DiInputTransformation::LocalToZonedDateTime)
        ));
        requests.push(TransformationRequest::new_di(
            TransformationDefinition::new(3,
                                          TransformationType::DoubleInput,
                                          String::from("isDay")),
            DoubleInputTransformationDefinition::new(
                3,
                String::from("zonedDateTimeValue"),
                String::from("geoArg"),
                DiInputTransformation::IsDay)
        ));
        Transformer::transform(payload, &requests)
    }

}


