use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};

use crate::context;
use crate::handlers::forwarding::{forward_request, DataType, HTTPMethod};
use crate::handlers::verify_token::verify_token;

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ai")
            .route("/chat", web::post().to(chat))
            .route("/documents/upload", web::post().to(documents_upload))
            .route("/documents", web::post().to(documents_create))
            .route("/documents", web::get().to(documents_list))
            .route("/documents/{id}", web::get().to(document_get))
            .route("/documents/{id}", web::delete().to(document_delete))
            .route("/documents/search", web::post().to(documents_search))
            .route("/user-documents/upload", web::post().to(user_documents_upload))
            .route("/conversations", web::get().to(conversations_list))
            .route("/conversations/{id}", web::get().to(conversation_get))
            .route("/conversations/{id}", web::delete().to(conversation_delete)),
    );
}

async fn authenticated(
    req: HttpRequest,
    body: web::Bytes,
    ctx: web::Data<Arc<context::Context>>,
    endpoint: &'static str,
    method: HTTPMethod,
    data_type: DataType,
) -> HttpResponse {
    let claims = match verify_token(req.clone()).await {
        Ok(claims)
            if claims
                .context
                .as_ref()
                .and_then(|context| context.scope.tenant_id())
                .is_some() =>
        {
            claims
        },
        Ok(_) => return HttpResponse::Unauthorized().body("El token no contiene tenant_id"),
        Err(error) => return error.error_response(),
    };

    forward_request("ai", endpoint, method, data_type, Some(claims))(req, body, ctx).await
}

macro_rules! ai_handler {
    ($name:ident, $endpoint:expr, $method:expr, $data_type:expr) => {
        async fn $name(req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>) -> HttpResponse {
            authenticated(req, body, ctx, $endpoint, $method, $data_type).await
        }
    };
}

ai_handler!(chat, "/ai/chat", HTTPMethod::POST, DataType::JSON);
ai_handler!(documents_upload, "/ai/documents/upload", HTTPMethod::POST, DataType::Multipart);
ai_handler!(documents_create, "/ai/documents", HTTPMethod::POST, DataType::JSON);
ai_handler!(documents_list, "/ai/documents", HTTPMethod::GET, DataType::None);
ai_handler!(document_get, "/ai/documents/{id}", HTTPMethod::GET, DataType::None);
ai_handler!(document_delete, "/ai/documents/{id}", HTTPMethod::DELETE, DataType::None);
ai_handler!(documents_search, "/ai/documents/search", HTTPMethod::POST, DataType::JSON);
ai_handler!(
    user_documents_upload,
    "/ai/user-documents/upload",
    HTTPMethod::POST,
    DataType::Multipart
);
ai_handler!(conversations_list, "/ai/conversations", HTTPMethod::GET, DataType::None);
ai_handler!(conversation_get, "/ai/conversations/{id}", HTTPMethod::GET, DataType::None);
ai_handler!(
    conversation_delete,
    "/ai/conversations/{id}",
    HTTPMethod::DELETE,
    DataType::None
);
