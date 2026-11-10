use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

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
                "/organization/{organization_id}/compare",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió POST /api/analytics/organization/{{organization_id}}/compare");
                    forward_request(
                        "analytics",
                        "/analytics/organization/{organization_id}/compare",
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
    let claims = match verify_token(req.clone()).await {
        Ok(claims) => claims,
        Err(error) => return error.error_response(),
    };
    let mut event = body.into_inner();

    // El ámbito confiable procede exclusivamente del contexto firmado.
    let tenant_id = claims.context.as_ref().and_then(|context| context.scope.tenant_id());
    let Some(tenant_id) = tenant_id else {
        return HttpResponse::Forbidden().body("El contexto no contiene tenant_id");
    };
    event.tenant_id = Some(tenant_id.to_string());

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
