use actix_web::{get, App, HttpServer, Responder, post, web};
use serde::Deserialize;

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello world!")
}

#[derive(Deserialize)]
struct Args {
    name: String
}

#[post("/greet_name")]
async fn greet_name(args: web::Json<Args>) -> impl Responder {
    format!("Hello {}", args.name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).service(greet_name)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
