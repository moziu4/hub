use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};

use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    ctx: web::Data<Arc<context::Context>>,
    endpoint: &'static str,
    method: HTTPMethod,
    data_type: DataType,
) -> HttpResponse {
    let claims = match verify_token(req.clone()).await {
        Ok(c) => Some(c),
        Err(e) => return HttpResponse::from_error(e),
    };

    forward_request("billing", endpoint, method, data_type, claims)(req, body, ctx).await
}

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/plans")
            .route(
                "",
                web::post()
                    .to(|req, body, ctx| forward(req, body, ctx, "/api/plans", HTTPMethod::POST, DataType::JSON)),
            )
            .route(
                "/all",
                web::get()
                    .to(|req, body, ctx| forward(req, body, ctx, "/api/plans/all", HTTPMethod::GET, DataType::None)),
            )
            .route(
                "/{id}",
                web::get()
                    .to(|req, body, ctx| forward(req, body, ctx, "/api/plans/{id}", HTTPMethod::GET, DataType::None)),
            )
            .route(
                "/{id}",
                web::patch()
                    .to(|req, body, ctx| forward(req, body, ctx, "/api/plans/{id}", HTTPMethod::PATCH, DataType::JSON)),
            )
            .route(
                "/{id}",
                web::delete().to(|req, body, ctx| {
                    forward(req, body, ctx, "/api/plans/{id}", HTTPMethod::DELETE, DataType::None)
                }),
            ),
    );

    cfg.service(
        web::scope("/api/billing")
            .route(
                "/organization-subscriptions/create",
                web::post().to(|req, body, ctx| {
                    forward(
                        req,
                        body,
                        ctx,
                        "/api/billing/organization-subscriptions/create",
                        HTTPMethod::POST,
                        DataType::JSON,
                    )
                }),
            )
            .route(
                "/organization-subscriptions/all",
                web::get().to(|req, body, ctx| {
                    forward(
                        req,
                        body,
                        ctx,
                        "/api/billing/organization-subscriptions/all",
                        HTTPMethod::GET,
                        DataType::None,
                    )
                }),
            )
            .route(
                "/customer-subscriptions/create",
                web::post().to(|req, body, ctx| {
                    forward(
                        req,
                        body,
                        ctx,
                        "/api/billing/customer-subscriptions/create",
                        HTTPMethod::POST,
                        DataType::JSON,
                    )
                }),
            )
            .route(
                "/customer-subscriptions/all",
                web::get().to(|req, body, ctx| {
                    forward(
                        req,
                        body,
                        ctx,
                        "/api/billing/customer-subscriptions/all",
                        HTTPMethod::GET,
                        DataType::None,
                    )
                }),
            )
            .route(
                "/invoices/create",
                web::post().to(|req, body, ctx| {
                    forward(req, body, ctx, "/api/billing/invoices/create", HTTPMethod::POST, DataType::JSON)
                }),
            )
            .route(
                "/invoices/all",
                web::get().to(|req, body, ctx| {
                    forward(req, body, ctx, "/api/billing/invoices/all", HTTPMethod::GET, DataType::None)
                }),
            )
            .route(
                "/payments/create",
                web::post().to(|req, body, ctx| {
                    forward(req, body, ctx, "/api/billing/payments/create", HTTPMethod::POST, DataType::JSON)
                }),
            )
            .route(
                "/payments/all",
                web::get().to(|req, body, ctx| {
                    forward(req, body, ctx, "/api/billing/payments/all", HTTPMethod::GET, DataType::None)
                }),
            ),
    );
}
