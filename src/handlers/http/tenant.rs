use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/agency")
            .route(
                "",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding POST to /api/agency");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/agency",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/agency/all");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/agency/all",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/agency/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/agency/{id}",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding PATCH to /api/agency/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/agency/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding DELETE to /api/agency/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/agency/{id}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            ),
    );

    cfg.service(
        web::scope("/api/tenant")
            .route(
                "",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding POST to /api/tenant");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/tenant",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/tenant/all");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/tenant/all",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/by_agency",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding POST to /api/tenant/by_agency");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/tenant/by_agency",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/tenant/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/tenant/{id}",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding PATCH to /api/tenant/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/tenant/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding DELETE to /api/tenant/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/tenant/{id}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            ),
    );

    cfg.service(
        web::scope("/api/plan")
            .route(
                "",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding POST to /api/plan");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/plan",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/plan/all");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/plan/all",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )

            .route(
                "/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/plan/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/plan/{id}",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding PATCH to /api/plan/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/plan/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{id}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding DELETE to /api/plan/{{id}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/plan/{id}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            ),
    );
}
