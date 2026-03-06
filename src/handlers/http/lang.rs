use std::sync::Arc;
use actix_web::{web, HttpRequest};
use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/lang")
            .route(
                "/statics/{group}",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Actix recibió la solicitud GET a /lang/group");
                    forward_request(
                        "lang",
                        "/api/statics/{group}",
                        HTTPMethod::GET,
                        DataType::None,
                    )(req, body, ctx).await
                }),
            )
    );
}