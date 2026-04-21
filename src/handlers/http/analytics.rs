use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/analytics")
            .route(
                "/events",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /api/analytics/events");
                    forward_request(
                        "analytics",
                        "/analytics/events",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/summary",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /api/analytics/summary");
                    forward_request(
                        "analytics",
                        "/analytics/summary",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/pages",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /api/analytics/pages");
                    forward_request(
                        "analytics",
                        "/analytics/pages",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/agency/{agency_id}/compare",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /api/analytics/agency/{{agency_id}}/compare");
                    forward_request(
                        "analytics",
                        "/analytics/agency/{agency_id}/compare",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
    );
}
