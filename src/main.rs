use std::{thread, time};
use std::sync::Mutex;

use actix::{Actor, Addr, System};
use actix_web::{App, http, HttpRequest, HttpServer, middleware};
use actix_web::web::{Data, get, resource};
use env_logger;

use crate::app::agents::core::{Agent, AgentAddress};
use crate::app::behavior::node::BTNodeAddress;
use crate::app::behavior::tick::Tick;
use crate::app::behavior::tree::BehaviorTree;

mod app;

async fn example(data: Data<Mutex<Agents>>) -> String {
    let agent = Agent::new(AgentAddress::new(1, 1),
                           BehaviorTree::new(1, BTNodeAddress::new(1, 1)))
        .start();
    let mut agents = data.lock().unwrap();
    agents.push(agent);

    let mut response = String::new();

    for address in &agents.agents {
        let response_one = address.send(Tick);
        let response_two = address.send(Tick);
        let response_three = address.send(Tick);
        response.push_str(format!("Welcome: {:?}, {:?}, {:?}",
                                  response_one.await.unwrap_err(),
                                  response_two.await.unwrap_err(),
                                  response_three.await.unwrap_err())
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
