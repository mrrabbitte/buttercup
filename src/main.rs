use std::{thread, time};

use actix::{Actor, System};
use actix_web::{App, http, HttpRequest, HttpServer, middleware};
use actix_web::web::{get, resource};
use env_logger;

use crate::app::agents::core::{Agent, AgentAddress};
use crate::app::behavior::node::BTNodeAddress;
use crate::app::behavior::tick::Tick;
use crate::app::behavior::tree::BehaviorTree;

mod app;

async fn example() -> String {
    let address =
        Agent::new(AgentAddress::new(1, 1),
                   BehaviorTree::new(1, BTNodeAddress::new(1, 1)))
            .start();
    let response_one = address.send(Tick);
    let response_two = address.send(Tick);
    let response_three = address.send(Tick);
    let response = format!("Welcome: {:?}, {:?}, {:?}",
            response_one.await.unwrap_err(),
            response_two.await.unwrap_err(),
            response_three.await.unwrap_err());
    address.send(Tick).await;
    response
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .service(
                resource("/test")
                    .route(
                        get().to(example))
            )
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:7777")?.run().await
}
