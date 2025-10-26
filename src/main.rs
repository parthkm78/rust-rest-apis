mod db;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let client = db::init_db().await.expect("Failed to connect to database");

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(Logger::default())
            .route("/health", web::get().to(handlers::health_check))
            .route("/users", web::get().to(handlers::get_users))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
