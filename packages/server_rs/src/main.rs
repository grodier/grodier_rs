use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
