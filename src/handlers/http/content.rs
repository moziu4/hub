use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::http::tenant::get_tenant_info;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/content")
            .route(
                "/slug/{slug}",
                web::get().to(get_tenant_info),
            )
            .route(
                "/catalog",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió GET /content/catalog");
                    forward_request(
                        "content",
                        "/api/content/catalog",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/new",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /content/new");
                    forward_request(
                        "content",
                        "/api/content/new",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió GET /content/all");
                    forward_request(
                        "content",
                        "/api/content/all",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/active",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió GET /content/active");
                    forward_request(
                        "content",
                        "/api/content/active",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/id/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió GET /content/id/");
                    forward_request(
                        "content",
                        "/api/content/id/{id}",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió PATCH /content/id");
                    forward_request(
                        "content",
                        "/api/content/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió DELETE /content/id");
                    forward_request(
                        "content",
                        "/api/content/{id}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            ),
    );
}
