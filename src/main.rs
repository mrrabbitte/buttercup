#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derivative;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix::{Actor, Addr};
use actix_web::{App, http, HttpRequest, HttpServer, middleware};
use actix_web::web::{Data, get, resource};
use dashmap::DashMap;
use env_logger;
use uuid::Uuid;

use crate::app::agents::core::{Agent, AgentAddress};
use crate::app::behavior::context::BTNodeExecutionContext;
use crate::app::behavior::node::BehaviorTreeNode;
use crate::app::behavior::node::action::logging::PrintLogActionNode;
use crate::app::behavior::node::action::wait::WaitDurationActionNode;
use crate::app::behavior::node::composite::fallback::FallbackCompositeNode;
use crate::app::behavior::node::decorator::reactive::ReactiveConditionDecoratorNode;
use crate::app::behavior::tree::BehaviorTree;
use crate::app::blackboards::service::BlackboardService;
use crate::app::conditions::ConditionExpressionWrapper;

mod app;

async fn reactive_tick(data: Data<Arc<BTNodeExecutionContext>>) -> String {
    let node = FallbackCompositeNode::new(
        1, vec![
            ReactiveConditionDecoratorNode::new(
                2,
                Box::new(
                    WaitDurationActionNode::new(
                        3,
                        Duration::from_millis(10000))
                        .into()),
                ConditionExpressionWrapper::always_true())
                .into(),
            PrintLogActionNode::new(
                4,
                "Looks like reactive node returned failure, so cool!.".to_string())
                .into()]
    );

    data.get_reactive_service().initialize_node(&2);

    format!("Got: {:?}", node.tick(data.as_ref()).await)
}

async fn abort_tick(data: Data<Arc<BTNodeExecutionContext>>) -> String {
    data.get_reactive_service().abort(&2);

    "Aborted".to_owned()
}

async fn example(data: Data<Mutex<Agents>>) -> String {
    let mut agents = data.lock().unwrap();
    let n_agents = agents.len() as i32;
    let agent = Agent::new(AgentAddress::new(n_agents, n_agents as u32),
                           BehaviorTree::new(n_agents,
                                             Arc::new(Default::default()),
                                             PrintLogActionNode::new(
                                                 n_agents, "hello".to_owned())
                                                 .into()));
    agents.push(agent);

    let mut response = String::new();

    for agent in &agents.agents {
        let response_one = agent.tick().await;
        let response_two = agent.tick().await;
        let response_three = agent.tick().await;
        response.push_str(format!("Welcome: {:?}, {:?}, {:?}",
                                  response_one,
                                  response_two,
                                  response_three)
            .as_str());
        agent.tick().await;
    }
    response.to_owned()
}

pub struct Agents {

    agents: Vec<Agent>

}

impl Agents {

    pub fn push(&mut self, agent: Agent) {
        self.agents.push(agent)
    }

    pub fn len(&self) -> usize {
        self.agents.len()
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::init();

    let agents_data =  Data::new(Mutex::new(Agents{ agents: vec![]}));
    let context =
        Data::new(Arc::new(BTNodeExecutionContext::default()));

    HttpServer::new(move || {
        App::new()
            .app_data(agents_data.clone())
            .app_data(context.clone())
            .service(
                resource("/test")
                    .route(
                        get().to(example))
            )
            .service(
                resource("/tick")
                    .route(
                        get().to(reactive_tick)))
            .service(
                resource("/abort")
                    .route(
                        get().to(abort_tick)))
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:7777")?.run().await
}
