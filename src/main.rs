use dotenv::dotenv;
use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Arc;
use actix_web::http::header::{HeaderValue};
use env_logger::Env;
use reqwest::Client;
use hub::context::Context;
use hub::handlers::{http::{user, shop, lang}};
use actix_web::dev::RequestHead;
use hub::handlers::http::{content, tenant};

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

    let context = Arc::new(Context::new(client.clone()));

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
            .allow_any_header() 
            .supports_credentials()
            .max_age(3600);

        
        App::new()
            .wrap(middleware::Logger::default()) 
            .wrap(cors) 
            .app_data(web::Data::new(context.clone())) 
            .configure(user::service) 
            .configure(shop::service) 
            .configure(lang::service)
            .configure(content::service)
            .configure(tenant::service)
    })
        .bind(&bind_address)?
        .run()
        .await
}