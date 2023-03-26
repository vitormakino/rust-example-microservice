use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, middleware::Logger};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server: go to http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(greet)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}