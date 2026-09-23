use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};

#[derive(Debug, Serialize, Deserialize)]
struct AnalyticsEvent {
    #[serde(default)]
    tenant_id: Option<String>,
    event_type: String,
    #[serde(default)]
    metadata: Value,
    #[serde(flatten)]
    extra: Value,
}

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/analytics")
            .route(
                "/events",
                web::post().to(post_events),
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
            .route(
                "/tenant/{tenant_id}/events-by-type",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /api/analytics/tenant/{{tenant_id}}/events-by-type");
                    forward_request(
                        "analytics",
                        "/analytics/tenant/{tenant_id}/events-by-type",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
        
    );
}

async fn post_events(
    req: HttpRequest,
    body: web::Json<AnalyticsEvent>,
    ctx: web::Data<Arc<context::Context>>,
) -> HttpResponse {
    let mut event = body.into_inner();

    // 1. Obtener tenant_id del header o del body
    let tenant_id = req.headers()
        .get("X-Tenant-Id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| event.tenant_id.clone());

    if let Some(tid) = tenant_id {
        event.tenant_id = Some(tid);
    }

    println!("Evento de analítica recibido: {} para tenant: {:?}", event.event_type, event.tenant_id);

    // 2. Procesamiento asíncrono vía NATS
    let nats_client = ctx.nats_client.clone();
    let event_payload = serde_json::to_vec(&event).unwrap_or_default();

    // No esperamos el resultado del publish para responder rápido
    tokio::spawn(async move {
        if let Err(e) = nats_client.publish("analytics.events", event_payload.into()).await {
            eprintln!("Error publicando evento en NATS: {}", e);
        }
    });

    // 3. Responder de inmediato 202 Accepted
    HttpResponse::Accepted().finish()
}
