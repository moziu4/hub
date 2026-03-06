use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route(
                "/all",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /users/all");
                    if let Err(err) = verify_token(req.clone()) {
                        eprintln!("Error de token: {:?}", err);
                        return err.error_response();
                    }

                    forward_request(
                        "user",
                        "/api/users/all",
                        HTTPMethod::GET,
                        DataType::None,
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
                    )(req, body, ctx).await
                }),
            )


            // .route(
            //     "/username/{una}",
            //     web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
            //         println!("Actix recibió la solicitud GET a /users/username/");
            //         forward_request(
            //             "user",
            //             "/api/users/username/{una}",
            //             HTTPMethod::GET,
            //             DataType::None,
            //         )(req, body, ctx).await
            //     }),
            // )
            // .route(
            //     "/userid/{id}",
            //     web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
            //         println!("Actix recibió la solicitud GET a /users/userid/");
            //         forward_request(
            //             "user",
            //             "/api/users/userid/{id}",
            //             HTTPMethod::GET,
            //             DataType::None,
            //         )(req, body, ctx).await
            //     }),
            // )
            .route(
                "/catalogs/{id}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /api/catalogs");
                    forward_request(
                        "user",
                        "/api/catalogs/import",
                        HTTPMethod::POST,
                        DataType::None,
                    )(req, body, ctx).await
                }),
            ),
        
    );
}