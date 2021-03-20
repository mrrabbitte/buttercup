use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix::{Actor, Addr, Arbiter};
use actix_web::{App, http, HttpRequest, HttpServer, middleware};
use actix_web::{post, Responder, web};
use actix_web::web::{Data, get, resource};
use dashmap::DashMap;
use env_logger;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use buttercup_agents::service::AgentService;
use buttercup_blackboards::LocalBlackboardService;
use buttercup_bts::context::{BTNodeContextService, BTNodeExecutionContextHolder};
use buttercup_endpoints::endpoints::EndpointService;
use buttercup_values::ValuesPayload;

pub mod test_utils;


#[post("/values/{name}/{value}")]
async fn add_variable_value(
    data: Data<Arc<EndpointService>>,
    web::Path((name, value)): web::Path<(String, String)>) -> impl Responder {

    serde_json::to_string(
        &data.accept_value_changes(&Uuid::from_u128(1),
                                   ValuesPayload::singleton(name, value.into())))
        .unwrap()
}

#[derive(Serialize, Deserialize)]
struct TreeId {

    id: i32

}

#[post("/agents")]
async fn build_new_agent(agent_service: Data<Arc<AgentService>>,
                         tree_id: web::Json<TreeId>) -> impl Responder {
    format!("{:?}", agent_service
        .build_new_agent(&tree_id.0.id)
        .map(|id| id.to_string()))
}

#[post("/agents/{agent_id}/start")]
async fn start_agent(agent_service: Data<Arc<AgentService>>,
                     agent_id: web::Path<Uuid>) -> impl Responder {
    format!("{:?}", agent_service
        .start_agent_by_id(&agent_id.0)
    )
}

#[post("/agents/{agent_id}/stop")]
async fn stop_agent(agent_service: Data<Arc<AgentService>>,
                    agent_id: web::Path<Uuid>) -> impl Responder {
    format!("{:?}", agent_service
        .stop_agent_by_id(&agent_id.0)
    )
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let blackboard_service: Arc<LocalBlackboardService> =
        Arc::new(LocalBlackboardService::default());
    let endpoint_service = Arc::new(EndpointService::new(
        Arbiter::new(),
        blackboard_service.clone()
    ));

    let context_service =
        Arc::new(BTNodeContextService::new(endpoint_service.clone(),
                                           blackboard_service.clone()));

    let agent_service =
        test_utils::build_test_agent_service(context_service.clone());

    let agent_service_data = Data::new(Arc::new(agent_service));
    let endpoints_service_data = Data::new(endpoint_service);

    HttpServer::new(move || {
        App::new()
            .app_data(endpoints_service_data.clone())
            .app_data(agent_service_data.clone())
            .service(add_variable_value)
            .service(build_new_agent)
            .service(start_agent)
            .service(stop_agent)
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:7777")?.run().await
}
