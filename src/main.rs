use actix_web::{App, HttpResponse, HttpServer, post, Responder, web};
use actix_web::web::{Json, Path};
use serde_json::Value;

mod arguments;
mod content;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/video/{id}")]
async fn index2(path: Path<u32>,
                body: Json<Value>) -> impl Responder {
    let values = body.into_inner();
    HttpResponse::Ok().body(format!("Hello world again: {:?}, {:?}",
                                    body.into_inner(), path.into_inner()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index2)
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}