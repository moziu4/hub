use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use redis::AsyncCommands;
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

                    let host = req.connection_info().host().to_string();
                    let response = forward_request(
                        "tenant",
                        "/api/tenant",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx.clone()).await;

                    if response.status().is_success() {
                        if let Ok(mut conn) = ctx.redis_client.get_multiplexed_async_connection().await {
                            let redis_key = format!("tenant:domain:{}", host);
                            let _: Result<(), _> = conn.del(&redis_key).await;
                            println!("Invalidada caché para el host: {}", host);
                        }
                    }
                    response
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
                "/info",
                web::get().to(get_tenant_info),
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

                    let host = req.connection_info().host().to_string();
                    let response = forward_request(
                        "tenant",
                        "/api/tenant/{id}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx.clone()).await;

                    if response.status().is_success() {
                        if let Ok(mut conn) = ctx.redis_client.get_multiplexed_async_connection().await {
                            let redis_key = format!("tenant:domain:{}", host);
                            let _: Result<(), _> = conn.del(&redis_key).await;
                            println!("Invalidada caché para el host: {}", host);
                        }
                    }
                    response
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

                    let host = req.connection_info().host().to_string();
                    let response = forward_request(
                        "tenant",
                        "/api/tenant/{id}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        claims,
                    )(req, body, ctx.clone()).await;

                    if response.status().is_success() {
                        if let Ok(mut conn) = ctx.redis_client.get_multiplexed_async_connection().await {
                            let redis_key = format!("tenant:domain:{}", host);
                            let _: Result<(), _> = conn.del(&redis_key).await;
                            println!("Invalidada caché para el host: {}", host);
                        }
                    }
                    response
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

    cfg.service(
        web::scope("/api/menus")
            .route(
                "/public",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/menus/public");
                    forward_request(
                        "tenant",
                        "/api/menus/public",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/menus");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/menus",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding POST to /api/menus");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/menus",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{name}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding PATCH to /api/menus/{{name}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/menus/{name}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{name}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding DELETE to /api/menus/{{name}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/menus/{name}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            ),
    );

    cfg.service(
        web::scope("/api/{id}/menus")
            .route(
                "/public",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/{{id}}/menus/public");
                    forward_request(
                        "tenant",
                        "/api/{id}/menus/public",
                        HTTPMethod::GET,
                        DataType::None,
                        None,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "",
                web::get().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding GET to /api/{{id}}/menus");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/{id}/menus",
                        HTTPMethod::GET,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "",
                web::post().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding POST to /api/{{id}}/menus");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/{id}/menus",
                        HTTPMethod::POST,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{name}",
                web::patch().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding PATCH to /api/{{id}}/menus/{{name}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/{id}/menus/{name}",
                        HTTPMethod::PATCH,
                        DataType::JSON,
                        claims,
                    )(req, body, ctx).await
                }),
            )
            .route(
                "/{name}",
                web::delete().to(|req: HttpRequest, body: web::Bytes, ctx: web::Data<Arc<context::Context>>| async move {
                    println!("Forwarding DELETE to /api/{{id}}/menus/{{name}}");
                    let claims = match verify_token(req.clone()).await {
                        Ok(c) => Some(c),
                        Err(e) => return HttpResponse::from_error(e),
                    };
                    forward_request(
                        "tenant",
                        "/api/{id}/menus/{name}",
                        HTTPMethod::DELETE,
                        DataType::None,
                        claims,
                    )(req, body, ctx).await
                }),
            ),
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct CombinedResponse {
    tenant: serde_json::Value,
    content: serde_json::Value,
}

pub async fn get_tenant_info(
    req: HttpRequest,
    ctx: web::Data<Arc<context::Context>>,
) -> HttpResponse {
    // 0. Resolución de Idioma
    // Orden: Cookie 'lang' > Header 'X-Language' > Header 'Accept-Language'
    let lang = req.cookie("lang")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers().get("X-Language")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
        })
        .or_else(|| {
            req.headers().get("Accept-Language")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next()) // Tomar el primero de la lista
                .map(|s| s.trim().to_string())
        });

    // Intentar obtener el tenant_id de las cabeceras primero (X-Tenant-Id)
    let tenant_id_header = req.headers().get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    let host = tenant_id_header.unwrap_or_else(|| req.connection_info().host().to_string());
    println!("DEBUG: Host/TenantId final utilizado: {}", host);
    println!("DEBUG: Idioma resuelto: {:?}", lang);

    // 0.1 Obtener el slug de la petición ANTES para usarlo en la clave de caché
    let slug = req.match_info().get("slug")
        .map(|s| s.to_string())
        .or_else(|| {
            web::Query::<std::collections::HashMap<String, String>>::from_query(req.query_string()).ok()
                .and_then(|q| q.get("slug").cloned())
        })
        .unwrap_or_else(|| "default".to_string());
    
    // También buscar el host directamente en las cabeceras por si acaso para debug
    let header_host = req.headers().get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("no-header-host");
    println!("DEBUG: Host extraído de cabecera 'host': {}", header_host);

    // Nueva clave de caché solicitada: tenant:slug:idioma
    let redis_key = format!("cache:tenant:{}:{}:{}", host, slug, lang.as_deref().unwrap_or("default"));

    // 1. Intentar obtener de Redis (Opcional, si falla seguimos con la lógica normal)
    let mut conn = ctx.redis_client.get_multiplexed_async_connection().await.ok();

    if let Some(ref mut c) = conn {
        if let Ok(cached_data) = c.get::<_, String>(&redis_key).await {
            if let Ok(response_json) = serde_json::from_str::<CombinedResponse>(&cached_data) {
                println!("Sirviendo desde caché para el host: {}, slug: {} y lang: {}", host, slug, lang.as_deref().unwrap_or("default"));
                return HttpResponse::Ok()
                    .insert_header(("Vary", "X-Language"))
                    .json(response_json);
            }
        }
    } else {
        eprintln!("Aviso: No se pudo conectar a Redis. Continuando sin caché.");
    }

    println!("Caché miss (o Redis no disponible) para el host: {} y slug: {}. Llamando a microservicios.", host, slug);

    // 2. Llamada al servicio Tenant para obtener info por host
    let tenant_service_url = std::env::var("TENANT_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:4000".to_string());
    
    // Suponemos que el endpoint en tenant_service es /api/tenant/info y acepta el host por header o query
    // Según la descripción: "cojiendo el host con el host coja la informacion del tenant"
    let tenant_url = format!("{}/api/tenant/info", tenant_service_url);
    
    let mut tenant_request = ctx.client.get(&tenant_url)
        .header("X-Tenant-Host", host.clone()) // Pasamos el host por header antiguo
        .header("X-Tenant-Id", host.clone())   // Pasamos el host por header para conistencia con forwarding.rs
        .header("Host", host.clone());           // Pasamos el Host real por si el servicio lo usa para identificar el tenant

    if let Some(ref l) = lang {
        tenant_request = tenant_request.header("X-Language", l.clone()); // Enviamos el idioma normalizado si existe
    }

    let tenant_res = tenant_request.send().await;

    let mut tenant_data: serde_json::Value = match tenant_res {
        Ok(res) if res.status().is_success() => {
            match res.json().await {
                Ok(data) => {
                    println!("Respuesta recibida del tenant_service: {:?}", data);
                    data
                },
                Err(_) => return HttpResponse::InternalServerError().body("Error parseando respuesta del tenant service"),
            }
        },
        Ok(res) => return HttpResponse::build(actix_web::http::StatusCode::from_u16(res.status().as_u16()).unwrap()).body("Error en tenant service"),
        Err(e) => return HttpResponse::BadGateway().body(format!("Error comunicándose con tenant service: {}", e)),
    };

    let content_service_url = std::env::var("CONTENT_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:4001".to_string());

    let default_lang = tenant_data.get("default_language")
        .and_then(|l| l.as_str())
        .unwrap_or("es-Es")
        .to_string();

    // Procesar menús si existen
    if let Some(menus) = tenant_data.get_mut("menus").and_then(|m| m.as_array_mut()) {
        for menu in menus {
            if let Some(items) = menu.get_mut("items").and_then(|i| i.as_array_mut()) {
                let mut stack: Vec<&mut serde_json::Value> = items.iter_mut().collect();
                
                while let Some(item) = stack.pop() {
                    // 1. Procesar el ítem actual
                    if let Some(item_type) = item.get("item_type") {
                        let type_str = item_type.get("type").and_then(|t| t.as_str()).unwrap_or("");
                        match type_str {
                            "page" | "feature" => {
                                let group_id_val = item.get("group_id")
                                    .or_else(|| item.get("item_type").and_then(|it| it.get("value")).and_then(|v| v.get("group_id")));
                                
                                if let Some(group_id) = group_id_val.and_then(|v| {
                                    v.as_str().map(|s| s.to_string())
                                        .or_else(|| v.get("$oid").and_then(|o| o.as_str()).map(|s| s.to_string()))
                                }) {
                                    let url = format!("{}/api/content/group/{}", content_service_url, group_id);
                                    let mut req = ctx.client.get(&url);
                                    let mut debug_query = vec![];
                                    if let Some(ref l) = lang {
                                        req = req.header("Accept-Language", l.clone())
                                                 .query(&[("locale", l.as_str())]);
                                        debug_query.push(("locale", l.as_str()));
                                    }
                                    println!("DEBUG: Llamando a Content Group URL: {} con query: {:?}", url, debug_query);
                                    if let Ok(res) = req.send().await {
                                        if res.status().is_success() {
                                            if let Ok(content_info) = res.json::<serde_json::Value>().await {
                                                if let Some(item_obj) = item.as_object_mut() {
                                                    // Intentar obtener label de 'label', 'title' o 'content.title'
                                                    let label = content_info.get("label")
                                                        .or_else(|| content_info.get("title"))
                                                        .or_else(|| content_info.get("content").and_then(|c| c.get("title")))
                                                        .cloned();
                                                    
                                                    // Intentar obtener slug de 'slug' o 'content.slug'
                                                    let slug_val = content_info.get("slug")
                                                        .or_else(|| content_info.get("content").and_then(|c| c.get("slug")))
                                                        .cloned();

                                                    if let Some(l) = label {
                                                        item_obj.insert("label".to_string(), l);
                                                    }
                                                    if let Some(s) = slug_val {
                                                        item_obj.insert("slug".to_string(), s);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "external_url" => {
                                let value = item.get("item_type").and_then(|it| it.get("value"));
                                let url = item.get("url")
                                    .or_else(|| value.and_then(|v| v.get("url")))
                                    .and_then(|u| u.as_str()).unwrap_or("").to_string();
                                
                                let labels = item.get("labels")
                                    .or_else(|| value.and_then(|v| v.get("labels")))
                                    .and_then(|l| l.as_array());
                                
                                let mut selected_label = "".to_string();
                                let requested_lang = lang.as_deref().unwrap_or(&default_lang);
                                
                                if let Some(labels) = labels {
                                    for label_entry in labels {
                                        let l_lang = label_entry.get("lang").and_then(|l| l.as_str()).unwrap_or("");
                                        let l_text = label_entry.get("label").and_then(|l| l.as_str()).unwrap_or("");
                                        if l_lang == requested_lang {
                                            selected_label = l_text.to_string();
                                            break;
                                        }
                                    }
                                    if selected_label.is_empty() && !labels.is_empty() {
                                        selected_label = labels[0].get("label").and_then(|l| l.as_str()).unwrap_or("").to_string();
                                    }
                                }
                                
                                if let Some(item_obj) = item.as_object_mut() {
                                    item_obj.insert("label".to_string(), serde_json::Value::String(selected_label));
                                    item_obj.insert("slug".to_string(), serde_json::Value::String(url));
                                }
                            },
                            _ => {}
                        }
                    }

                    // 2. Agregar hijos al stack para procesarlos
                    if let Some(children) = item.get_mut("children").and_then(|c| c.as_array_mut()) {
                        stack.extend(children.iter_mut());
                    }
                }
            }
        }
    }

    // Extraer tenant_id (asumimos que viene en la respuesta)
    let tenant_id = match tenant_data.get("tenant_id")
        .or_else(|| tenant_data.get("id"))
        .or_else(|| tenant_data.get("_id")) {
        Some(id) => {
            if let Some(oid) = id.get("$oid").and_then(|o| o.as_str()) {
                oid.to_string()
            } else if let Some(s) = id.as_str() {
                s.to_string()
            } else {
                id.to_string().replace("\"", "")
            }
        },
        None => {
            println!("Advertencia: No se encontró tenant_id en la respuesta del tenant service. La respuesta de content será nula.");
            "unknown".to_string()
        },
    };
    println!("ID del tenant extraído: {}", tenant_id);

    let mut content_data: serde_json::Value = if tenant_id == "unknown" {
        serde_json::Value::Null
    } else {
        println!("Llamando al servicio Content con tenant_id: {} y slug: {} y locale: {:?}", tenant_id, slug, lang);
        let content_url = format!("{}/api/content", content_service_url);
        
        let mut query_params = vec![
            ("tenant_id", tenant_id.as_str()),
            ("slug", slug.as_str()),
        ];
        if let Some(ref l) = lang {
            query_params.push(("locale", l.as_str()));
        }
        println!("DEBUG: Petición final a Content: URL={} params={:?}", content_url, query_params);

        let content_res = ctx.client.get(&content_url)
            .query(&query_params)
            .send()
            .await;

        match content_res {
            Ok(res) if res.status().is_success() => {
                match res.json().await {
                    Ok(data) => {
                        println!("Respuesta recibida del content_service: {:?}", data);
                        data
                    },
                    Err(_) => {
                        println!("Error parseando JSON del content_service.");
                        serde_json::Value::Null
                    },
                }
            },
            Ok(res) => {
                println!("Content service respondió con status no exitoso: {}", res.status());
                serde_json::Value::Null
            },
            Err(e) => {
                println!("Error en la llamada al content_service: {}", e);
                serde_json::Value::Null
            },
        }
    };

    // 4.1. Procesar Assets si existen en el contenido
    let assets_service_url = std::env::var("ASSETS_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:4004".to_string());

    if let Some(content_obj) = content_data.get_mut("content") {
        if let Some(blocks) = content_obj.get_mut("blocks").and_then(|b| b.as_array_mut()) {
            for block in blocks {
                if let Some(block_content) = block.get_mut("content") {
                    // Buscar image_item en el contenido del bloque
                    if let Some(image_item) = block_content.get_mut("image_item") {
                        let image_id = image_item.get("id").and_then(|id| id.as_str());

                        if let Some(id) = image_id {
                            let asset_url = format!("{}/assets/{}", assets_service_url, id);
                            println!("DEBUG: Llamando a Assets Service: {}", asset_url);
                            
                            match ctx.client.get(&asset_url).send().await {
                                Ok(asset_res) => {
                                    let status = asset_res.status();
                                    println!("DEBUG: Assets Service respondió con status: {}", status);
                                    if status.is_success() {
                                        if let Ok(asset_info) = asset_res.json::<serde_json::Value>().await {
                                            println!("DEBUG: Info del asset recibida: {:?}", asset_info);
                                            // Combinar la info del asset en el image_item
                                            if let Some(item_obj) = image_item.as_object_mut() {
                                                println!("DEBUG: image_item ANTES de combinar: {:?}", item_obj);
                                                if let Some(asset_obj) = asset_info.as_object() {
                                                    for (key, value) in asset_obj {
                                                        item_obj.insert(key.clone(), value.clone());
                                                    }
                                                }
                                                println!("DEBUG: image_item DESPUÉS de combinar: {:?}", item_obj);
                                            }
                                        } else {
                                            println!("DEBUG: No se pudo parsear el JSON de Assets Service");
                                        }
                                    } else {
                                        let error_text = asset_res.text().await.unwrap_or_else(|_| "No se pudo leer el cuerpo del error".to_string());
                                        println!("DEBUG: Error body de Assets Service: {}", error_text);
                                    }
                                },
                                Err(e) => {
                                    println!("DEBUG: Fallo total al enviar la petición al Assets Service: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let final_response = CombinedResponse {
        tenant: tenant_data,
        content: content_data,
    };

    println!("Respuesta combinada final: {:?}", final_response);

    // 5. Guardar en Redis (con un TTL de p.ej. 1 hora)
    if let Some(mut c) = conn {
        if let Ok(serialized) = serde_json::to_string(&final_response) {
            let _: Result<(), _> = c.set_ex(&redis_key, serialized, 3600).await;
        }
    }

    HttpResponse::Ok()
        .insert_header(("Vary", "X-Language"))
        .json(final_response)
}
