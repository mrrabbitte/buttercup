use std::collections::HashMap;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Json, Path};
use serde_json::Value;

use crate::app::arguments::ArgumentDefinition;
use crate::app::arguments::extraction::{ArgumentsExtractionInput, ArgumentValuesExtractionService};
use crate::app::transformations::transformer::TransformationService;
use crate::app::values::{ValuesPayload, ValueType};
use crate::app::values::extractors::ValueExtractionPolicy;

mod app;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/video/{id}")]
async fn index2(path: Path<u32>,
                body: Json<Value>) -> impl Responder {
    HttpResponse::Ok().body(format!("OK"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    TransformationService::initialize();
    HttpServer::new(|| {
        App::new()
            .service(index2)
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}