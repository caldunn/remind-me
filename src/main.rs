use actix_web::{get, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

