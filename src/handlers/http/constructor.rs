use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/constructor")
            .route(
                "/tenant/{tenant_id}/blocks",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /constructor/blocks/tenant/{{tenantId}}");
                    forward_request(
                        "constructor",
                        "/api/content/tenant/{tenant_id}/blocks",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/slug/{slug}",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /constructor/api/content/slug/{{slug}}");
                    forward_request(
                        "constructor",
                        "/api/content/slug/{slug}",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )

    );
}