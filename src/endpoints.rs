use std::ops::Deref;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Data, Json, Path};
use serde_json::Value;

use crate::app::pipeline::ContentPipelineService;
use crate::app::pipeline::core::ContentPipelineRequest;

#[post("{tenant_id}/pipeline/{id}")]
pub async fn pipeline(service: Data<ContentPipelineService>,
                      id: Path<(String, i32)>,
                      body: Json<Value>) -> impl Responder {
    let (tenant_id, pipeline_id) = id.into_inner();
    let request = ContentPipelineRequest::new(
        tenant_id, pipeline_id, body.deref());
    let response = service.evaluate(&request);
    println!("{:?}", response);
    HttpResponse::Ok().body(format!("OK"))
}