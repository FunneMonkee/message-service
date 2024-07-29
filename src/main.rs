use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed binding to localhost")
    .run()
    .await
}
