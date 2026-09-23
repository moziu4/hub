use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/assets")
            .route(
                "/products",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /assets/products");
                    forward_request(
                        "asset",
                        "/assets/products",
                        HTTPMethod::POST,
                        DataType::Multipart,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/images",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /assets/images");
                    forward_request(
                        "asset",
                        "/assets/images",
                        HTTPMethod::POST,
                        DataType::Multipart,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/contents",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /assets/contents");
                    forward_request(
                        "asset",
                        "/assets/contents",
                        HTTPMethod::POST,
                        DataType::Multipart,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}/{version}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió GET /assets/{{id}}/{{version}}");
                    forward_request(
                        "asset",
                        "/assets/{id}/{version}",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            ),

    );
}
