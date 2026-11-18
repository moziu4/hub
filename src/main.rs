use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use hub::context::Context;
use hub::handlers::{
    http::{ai, analytics, asset, billing, lang, tenant, user},
    nats,
};
use redis::Client as RedisClient;
use reqwest::Client;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let client = Arc::new(
        Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap(),
    );

    let redis_url = dotenv::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = Arc::new(RedisClient::open(redis_url.clone()).expect("Error configurando Redis"));
    println!("Cliente de Redis configurado para: {}", redis_url);

    let nats_url = dotenv::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string());
    let nats_client = Arc::new(
        async_nats::connect(nats_url)
            .await
            .expect("Error conectando a NATS"),
    );
    println!("Cliente de NATS configurado");

    let context = Arc::new(Context::new(client.clone(), redis_client.clone(), nats_client.clone()));

    // Iniciar el listener de NATS en una tarea separada
    let nats_ctx = context.clone();
    tokio::spawn(async move {
        nats::listeners::start_nats_listener(nats_ctx).await;
    });

    let bind_address = dotenv::var("HTTP_BIND").unwrap_or_else(|_| "localhost:8080".to_string());
    println!("Iniciando Hub en: {}", bind_address);
    println!("Configuración de servicios detectada:");
    for (key, value) in std::env::vars() {
        if key.ends_with("_SERVICE_URL") {
            println!("  {} = {}", key, value);
        }
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::HeaderName::from_static("x-language"),
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(context.clone()))
            .configure(user::service)
            .configure(lang::service)
            .configure(tenant::service)
            .configure(billing::service)
            .configure(asset::service)
            .configure(analytics::service)
            .configure(ai::service)
    })
    .bind(&bind_address)?
    .run()
    .await
}
