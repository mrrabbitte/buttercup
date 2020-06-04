#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, middleware, post, Responder, web};
use actix_web::web::{Json, Path};
use serde_json::Value;

use crate::app::arguments::ArgumentDefinition;
use crate::app::arguments::extraction::{ArgumentsExtractionInput, ArgumentValuesExtractionService};
use crate::app::transformations::transformer::TransformationService;
use crate::app::values::{ValuesPayload, ValueType};
use crate::app::values::extractors::ValueExtractionPolicy;
use crate::builder::content_pipeline_service;
use crate::test_utils::TestUtils;

mod app;
mod builder;
mod endpoints;
mod test_utils;
mod html_test_utils;
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    TransformationService::initialize();
    let pipeline_service = content_pipeline_service();
    let file_service = TestUtils::test_file_service();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(pipeline_service.clone())
            .service(endpoints::pipeline)
            .service(file_service.get_files())
    })
        .bind("localhost:7777")?
        .run()
        .await
}