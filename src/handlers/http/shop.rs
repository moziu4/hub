use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/shop")
            .route(
                "/products/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /shop/all");
                    forward_request(
                        "shop",
                        "/api/products/all",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/products/new",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud POST a /shop/new");
                    forward_request(
                        "shop",
                        "/api/products/new",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/products/id/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /shop/id/");
                    forward_request(
                        "shop",
                        "/api/products/id/{id}",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/products/paginated",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /shop/paginated");
                    forward_request(
                        "shop",
                        "/api/products/paginated",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/products/edit/{id}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud PATCH a /shop/edit/");
                    forward_request(
                        "shop",
                        "/api/products/edit/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/categories/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /shop/categories/all");
                    forward_request(
                        "shop",
                        "/api/categories/all",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/categories/new",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud POST a /shop/categories/new");
                    forward_request(
                        "shop",
                        "/api/categories/new",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/categories/id/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /shop/categories/id/");
                    forward_request(
                        "shop",
                        "/api/categories/id/{id}",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/categories/title/{title}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /shop/categories/title/");
                    forward_request(
                        "shop",
                        "/api/categories/title/{title}",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/categories/edit/{id}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud PATCH a /shop/categories/edit/");
                    forward_request(
                        "shop",
                        "/api/categories/edit/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )

            .route(
                "/categories/delete/{id}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud DELETE a /shop/categories/delete/");
                    forward_request(
                        "shop",
                        "/api/categories/delete/{id}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            ),

    );
}

