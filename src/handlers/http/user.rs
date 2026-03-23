use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route(
                "/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /users/all");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => c,
                        Err(err) => {
                            eprintln!("Error de token: {:?}", err);
                            return err.error_response();
                        }
                    };

                    // Ejemplo de validación de permiso: ver todos los usuarios (ID 24)
                    use crate::handlers::verify_token::has_permission;
                    let role = claims.role_id.to_string();

                    if !has_permission(&ctx.client, &role, 24).await {
                        return actix_web::HttpResponse::Forbidden().body("No tienes permiso para ver todos los usuarios");
                    }

                    forward_request(
                        "user",
                        "/api/users/all",
                        HTTPMethod::GET,
                        DataType::None,
                        Some(claims),
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/newuser",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud POST a /users/newuser");
                    forward_request(
                        "user",
                        "/api/users/newuser",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )

            .route(
                "/login",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud POST a /auth/login");
                    forward_request(
                        "user",
                        "/api/auth/login",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
    );

    cfg.service(
        web::scope("/api/user/catalogs")
            .route(
                "/import",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud POST a /api/catalogs/import");
                    forward_request(
                        "user",
                        "/api/catalogs/import",
                        HTTPMethod::POST,
                        DataType::JSON,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/roles",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /api/catalogs/roles");
                    forward_request(
                        "user",
                        "/api/catalogs/roles",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/document-types",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /api/catalogs/document-types");
                    forward_request(
                        "user",
                        "/api/catalogs/document-types",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            ),
    );
}