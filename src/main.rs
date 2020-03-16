use std::collections::HashMap;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Json, Path};
use serde_json::Value;

use crate::arguments::{ArgumentsProcessorInput, ArgumentValuesExtractor};
use crate::arguments::definition::ArgumentDefinition;
use crate::arguments::extractors::ValueExtractionPolicy;
use crate::values::ValueType;

mod arguments;
mod content;
mod values;

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
                       ArgumentDefinition::new(1,
                                               String::from("decimalArg"),
                                               ValueType::Decimal,
                                               ValueExtractionPolicy::Lax));
    let input = ArgumentsProcessorInput::new(
        definitions, &values);
    let extracted = ArgumentValuesExtractor::process(input);
    HttpResponse::Ok().body(format!("Request: {:?}, extracted {:?}",
                                    values, extracted))
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