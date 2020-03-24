use std::collections::HashMap;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Json, Path};
use serde_json::Value;

use crate::arguments::{ArgumentsExtractionInput, ArgumentValueExtractorError, ArgumentValuesExtractionService};
use crate::arguments::definition::ArgumentDefinition;
use crate::arguments::extractors::ValueExtractionPolicy;
use crate::transformations::TransformationService;
use crate::values::{ValuesPayload, ValueType};

mod arguments;
mod content;
mod values;
mod transformations;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/video/{id}")]
async fn index2(path: Path<u32>,
                body: Json<Value>) -> impl Responder {
    let values = body.into_inner();
    let mut definitions = HashMap::new();
    definitions.insert(String::from("dayOfWeekArg"),
                       ArgumentDefinition::new(1,
                                                String::from("dayOfWeekArg"),
                                                ValueType::DayOfWeek,
                        ValueExtractionPolicy::Lax));
    definitions.insert(String::from("decimalArg"),
                       ArgumentDefinition::new(2,
                                               String::from("decimalArg"),
                                               ValueType::Decimal,
                                               ValueExtractionPolicy::Lax));
    definitions.insert(String::from("geoArg"),
                       ArgumentDefinition::new(3,
                                               String::from("geoArg"),
                                               ValueType::GeoCoordinates,
                                               ValueExtractionPolicy::Lax));
    definitions.insert(String::from("dateTimeArg"),
                       ArgumentDefinition::new(3,
                                               String::from("dateTimeArg"),
                                               ValueType::LocalDateTime,
                                               ValueExtractionPolicy::Lax));
    definitions.insert(String::from("zoneArg"),
                       ArgumentDefinition::new(3,
                                               String::from("zoneArg"),
                                               ValueType::TimeZone,
                                               ValueExtractionPolicy::Lax));
    let input = ArgumentsExtractionInput::new(definitions, &values);
    let extracted =
        ArgumentValuesExtractionService::process(input);
    match extracted {
        Ok(payload) =>
            HttpResponse::Ok().body(
                format!("OK: {:?}", TransformationService::transform(&payload))),
        Err(err) => HttpResponse::BadRequest().body(
            format!("Request: {:?}, extracted {:?}", values, err)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index2)
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}