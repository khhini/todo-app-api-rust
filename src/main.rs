use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use todo_app_api_rust::{config::Config, handlers::hello};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if std::path::Path::new(".env").exists() {
        dotenv().ok();
    }

    let config = Config::from_env().unwrap_or_else(|err| {
        eprintln!("Failed to load config: {}", err);
        eprintln!("Using default config values.");
        Config::default()
    });

    println!(
        "Starting server at http://{}:{}",
        config.listener.host, config.listener.port
    );

    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(web::scope("/hello").configure(hello::handler_config)),
        )
    })
    .bind((config.listener.host, config.listener.port))?
    .run()
    .await
}
