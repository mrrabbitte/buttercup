use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix::{Actor, Addr, Arbiter};
use actix_web::{App, http, HttpRequest, HttpServer, middleware};
use actix_web::{post, Responder, web};
use actix_web::web::{Data, get, resource};
use dashmap::DashMap;
use env_logger;
use uuid::Uuid;

use buttercup_agents::Agent;
use buttercup_blackboards::{LocalBlackboard, LocalBlackboardService};
use buttercup_bts::context::BTNodeExecutionContext;
use buttercup_bts::context::reactive::ReactiveContext;
use buttercup_bts::node::{BehaviorTreeNode, BTNode};
use buttercup_bts::node::action::logging::PrintLogActionNode;
use buttercup_bts::node::action::wait::WaitDurationActionNode;
use buttercup_bts::node::composite::fallback::FallbackCompositeNode;
use buttercup_bts::node::decorator::reactive::ReactiveConditionDecoratorNode;
use buttercup_bts::node::root::one_off::OneOffRootBTNode;
use buttercup_bts::tree::BehaviorTree;
use buttercup_conditions::{ConditionExpression, ConditionExpressionWrapper,
                           LogicalExpression, RelationalExpression, RelationalExpressionSpecification};
use buttercup_conditions::relational::{EndsWithRelationalExpression, StartsWithRelationalExpression};
use buttercup_endpoints::endpoints::EndpointService;
use buttercup_values::{ValueHolder, ValuesPayload};

#[post("/values/{name}/{value}")]
async fn endpoint(data: Data<Arc<EndpointService>>,
                  web::Path((name, value)): web::Path<(String, String)>) -> impl Responder {
    serde_json::to_string(
        &data.accept_value_changes(&Uuid::from_u128(1),
                                   ValuesPayload::singleton(name, value.into())))
        .unwrap()
}

async fn reactive_tick(data: Data<Arc<BTNodeExecutionContext>>) -> String {
    let reactive_node: Arc<BTNode> = Arc::new(ReactiveConditionDecoratorNode::new(
        2,
        Arc::new(
            WaitDurationActionNode::new(
                3,
                Duration::from_millis(30000))
                .into()),
        ConditionExpressionWrapper::new(get_mock_condition_expression()))
        .into()
    );
    let node = FallbackCompositeNode::new(
        1, vec![
            reactive_node.clone(),]
            // Arc::new(PrintLogActionNode::new(
            //     4,
            //     "Looks like reactive node returned failure, so cool!.".to_string())
            //     .into()
            // )]
    );

    data.get_reactive_service().initialize_node(reactive_node.clone());

    format!("Got: {:?}", node.tick(data.as_ref()).await)
}

async fn abort_tick(data: Data<Arc<BTNodeExecutionContext>>) -> String {
    data.get_reactive_service().abort(&2);

    "Aborted".to_owned()
}

async fn example(data: Data<Mutex<Agents>>) -> String {
    let mut agents = data.lock().unwrap();
    let agent = Agent::new(1,
                           Arc::new(Default::default()),
                           Arc::new(
                               BehaviorTree::new(1,
                                                 OneOffRootBTNode::new(
                                                     1,
                                                     PrintLogActionNode::new(
                                                         1,
                                                         "hello".to_owned())
                                                         .into()
                                                 )
                                                     .into()
                               )
                           )
    );
    agents.push(agent);

    let mut response = String::new();

    for agent in &mut agents.agents {
        let response_one = agent.start().await;
        let response_two = agent.start().await;
        let response_three = agent.start().await;
        response.push_str(format!("Welcome: {:?}, {:?}, {:?}",
                                  response_one,
                                  response_two,
                                  response_three)
            .as_str());
        agent.start().await;
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
    let blackboard_service: Arc<LocalBlackboardService> =
        Arc::new(LocalBlackboardService::default());
    blackboard_service.create(Uuid::from_u128(1),
                                  "my-blackboard.bl".into());
    let reactive_service: Arc<ReactiveContext> = Arc::new(Default::default());
    let bt_node_context = Arc::new(BTNodeExecutionContext::new(
        Uuid::from_u128(1),
        blackboard_service.get(&Uuid::from_u128(1)).unwrap(),
        reactive_service.clone()));

    let context_data =
        Data::new(bt_node_context.clone());
    let endpoints_service_data =
        Data::new(
            Arc::new(EndpointService::new(
                Arbiter::new(),
                blackboard_service.clone(),
                Arc::new(move |changed|
                    reactive_service.handle_value_changes(
                        bt_node_context.as_ref(), changed)))
            ));
    let agents_data =  Data::new(Mutex::new(Agents{ agents: vec![]}));

    HttpServer::new(move || {
        App::new()
            .app_data(agents_data.clone())
            .app_data(context_data.clone())
            .app_data(endpoints_service_data.clone())
            .service(
                resource("/test")
                    .route(
                        get().to(example))
            )
            .service(endpoint)
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


fn get_mock_condition_expression() -> ConditionExpression {
    ConditionExpression::LogicalExpression(
        Box::new(
            LogicalExpression::And(
                vec![
                    ConditionExpression::RelationExpression(
                        RelationalExpression::EndsWith(
                            EndsWithRelationalExpression::new(
                                RelationalExpressionSpecification::NameAndLiteral(
                                    "var_name_1".to_owned(),
                                    "elka".to_owned().into()
                                )
                            )
                        )
                    ),
                    ConditionExpression::RelationExpression(
                        RelationalExpression::StartsWith(
                            StartsWithRelationalExpression::new(
                                RelationalExpressionSpecification::NameAndLiteral(
                                    "var_name_1".to_owned(),
                                    "mira".to_owned().into()
                                )
                            )
                        )
                    )
                ]
            )
        )
    )

}