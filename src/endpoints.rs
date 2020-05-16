use std::ops::Deref;

use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Data, Json, Path};
use serde_json::Value;

use crate::app::pipeline::ContentPipelineService;
use crate::app::pipeline::core::ContentPipelineRequest;

#[post("{tenant_id}/pipeline/{id}")]
pub async fn pipeline(service: Data<ContentPipelineService>,
                      tenant_id: Path<String>,
                      id: Path<i32>,
                      body: Json<Value>) -> impl Responder {
    let request = ContentPipelineRequest::new(
        tenant_id.into_inner(), id.into_inner(), body.deref());
    service.evaluate(&request);
    HttpResponse::Ok().body(format!("OK"))
}