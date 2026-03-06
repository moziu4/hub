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
use hub::handlers::http::content;

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

    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin: &HeaderValue, _req_head: &RequestHead| {
                let front_dev = dotenv::var("URL_FRONT_DEV").unwrap_or_default();
                let front_prod = dotenv::var("URL_FRONT").unwrap_or_default();

                if let Ok(origin_str) = origin.to_str() {
                    origin_str == front_dev || origin_str == front_prod
                } else {
                    false
                }
            })
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
    })
        .bind(&bind_address)?
        .run()
        .await
}