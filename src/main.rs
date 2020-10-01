#[macro_use]
extern crate lazy_static;

use std::sync::{Arc, Mutex};

use actix::{Actor, Addr};
use actix_web::{App, http, HttpRequest, HttpServer, middleware};
use actix_web::web::{Data, get, resource};
use dashmap::DashMap;
use env_logger;
use uuid::Uuid;

use crate::app::address::Address;
use crate::app::agents::core::{Agent, AgentAddress};
use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::BTNodeAddress;
use crate::app::behavior::tick::Tick;
use crate::app::behavior::tree::BehaviorTree;
use crate::app::blackboards::service::BlackboardService;

mod app;

async fn example(data: Data<Mutex<Agents>>) -> String {
    let agent = Agent::new(AgentAddress::new(1, 1),
                           BehaviorTree::new(1,
                                             BTNodeExecutionContext::new(
                                                 Uuid::from_u128(1),
                                                 Arc::new(
                                                     BlackboardService::new(
                                                         DashMap::new())))))
        .start();
    let mut agents = data.lock().unwrap();
    agents.push(agent);

    let mut response = String::new();

    for address in &agents.agents {
        let response_one = address.send(Tick).await;
        let response_two = address.send(Tick).await;
        let response_three = address.send(Tick).await;
        response.push_str(format!("Welcome: {:?}, {:?}, {:?}",
                                  response_one,
                                  response_two,
                                  response_three)
            .as_str());
        address.send(Tick).await;
    }
    response.to_owned()
}

pub struct Agents {

    agents: Vec<Addr<Agent>>

}

impl Agents {

    pub fn push(&mut self, addr: Addr<Agent>) {
        self.agents.push(addr)
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::init();

    let agents_data =  Data::new(Mutex::new(Agents{ agents: vec![]}));

    HttpServer::new(move || {
        App::new()
            .app_data(agents_data.clone())
            .service(
                resource("/test")
                    .route(
                        get().to(example))
            )
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:7777")?.run().await
}
