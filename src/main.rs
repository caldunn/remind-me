use actix_web::{get, web, App, HttpServer, Responder};
use rmlib::hello_world;
use rmlib::time_heap::connect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    hello_world();
    connect();
    lolwht();
    printkek(123);
    HttpServer::new(|| App::new().service(greet))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

pub fn lolwht() {
    println!("Kekl")
}
