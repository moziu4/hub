use actix_web::{HttpRequest, HttpResponse, web};
use reqwest::{Client, Method};
use serde_json;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::context::Context;
use crate::handlers::verify_token::Claims;

#[derive(Clone, Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Clone, Debug)]
pub enum DataType {
    JSON,
    None,
}

pub fn forward_request(
    service: &'static str,
    endpoint: &'static str,
    method: HTTPMethod,
    data_type: DataType,
    claims: Option<Claims>,
) -> impl Fn(HttpRequest, web::Bytes, web::Data<Arc<Context>>) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> {
    move |req, body, context| {
        let url = build_url(service, endpoint, &req);
        let client = context.client.clone();

        let cloned_data_type = data_type.clone();
        println!("{:?}", cloned_data_type);
        let cloned_method = method.clone();

        let mut headers: Vec<(String, String)> = req
            .headers()
            .iter()
            .filter(|(name, _)| !["host", "content-length", "accept-encoding", "connection", "upgrade", "referer", "origin"]
                .contains(&name.as_str()))
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
            .collect();

        if let Some(host) = req.headers().get("host") {
            if let Ok(host_str) = host.to_str() {
                headers.push(("X-Tenant-Id".to_string(), host_str.to_string()));
            }
        }

        // Inyectar datos del usuario autenticado si existen
        if let Some(ref c) = claims {
            headers.push(("X-User-Id".to_string(), c.sub.clone()));
            headers.push(("X-User-Role".to_string(), c.role_id.to_string()));
            
            if let Some(tenant_id) = c.tenant_id {
                headers.push(("X-User-Tenant-Id".to_string(), tenant_id.to_string()));
            }
            if let Some(agency_id) = c.agency_id {
                headers.push(("X-User-Agency-Id".to_string(), agency_id.to_string()));
            }
        }

        println!("Headers recibidos en hub_service:");
        for (key, value) in &headers {
            println!("  {}: {}", key, value);
        }

        Box::pin(async move {
            match perform_request(client.as_ref(), url, cloned_method, headers, body, cloned_data_type).await {
                Ok(response) => response,
                Err(err) => {
                    eprintln!("Error en el reenvío: {}", err);
                    HttpResponse::BadGateway().body(format!("Error comunicándose con el microservicio: {}", err))
                }
            }
        })
    }
}

fn build_url(service: &str, endpoint: &str, req: &HttpRequest) -> String {
    println!("DEBUG: build_url llamado para servicio: '{}'", service);
    let env_var_name = format!("{}_SERVICE_URL", service.to_uppercase());
    println!("DEBUG: Buscando variable de entorno: '{}'", env_var_name);
    
    let base_url = match std::env::var(&env_var_name) {
        Ok(val) => {
            println!("DEBUG: ¡Encontrada! {} = '{}'", env_var_name, val);
            val
        },
        Err(e) => {
            println!("DEBUG: Error leyendo {}: {:?}. Usando default.", env_var_name, e);
            "http://localhost:4000".to_string()
        }
    };
    
    println!("DEBUG: Base URL resultante: '{}'", base_url);
    let mut url = base_url + endpoint;
    println!("URL: {}", url);

    for (key, value) in req.match_info().iter() {
        url = url.replace(&format!("{{{}}}", key), value);
    }
    println!("URL final construida: {}", url);
    url
}

pub async fn perform_request(
    client: &Client,
    url: String,
    method: HTTPMethod,
    headers: Vec<(String, String)>,
    body: web::Bytes,
    data_type: DataType,
) -> Result<HttpResponse, reqwest::Error> {
    println!("Preparando solicitud hacia: {}", url);
    println!("Método HTTP: {:?}", method);
    println!("DataType: {:?}", data_type);

    let reqwest_method = match method {
        HTTPMethod::GET => Method::GET,
        HTTPMethod::POST => Method::POST,
        HTTPMethod::PUT => Method::PUT,
        HTTPMethod::DELETE => Method::DELETE,
        HTTPMethod::PATCH => Method::PATCH,
    };

    let mut request_builder = client.request(reqwest_method, &url);

    // Usar los headers extraídos
    for (name, value) in headers.clone() {
        if let (Ok(name), Ok(value)) = (
            reqwest::header::HeaderName::try_from(name.as_str()),
            reqwest::header::HeaderValue::try_from(value.as_str()),
        ) {
            request_builder = request_builder.header(name, value);
        } else {
            eprintln!("Error al convertir encabezado: {} => {}", name, value);
        }
    }

    // Manejar el cuerpo de la solicitud
    match data_type {
        DataType::JSON => {
            if !body.is_empty() {
                if let Ok(json_body) = serde_json::from_slice::<serde_json::Value>(&body) {
                    request_builder = request_builder.json(&json_body);
                } else {
                    eprintln!("Error: No se pudo deserializar el cuerpo en JSON.");
                }
            }
        }
        DataType::None => {}
    }

    let response = request_builder.send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "No se pudo obtener el error del microservicio.".to_string());


        let actix_status = actix_web::http::StatusCode::from_u16(status.as_u16())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        return Ok(HttpResponse::build(actix_status).body(error_body));
    }
    // Extraer información antes de consumir el cuerpo
    let status_code = response.status();
    let headers = response.headers().clone(); // Clonar los headers para reusarlos
    let response_body = response.bytes().await.unwrap_or_default(); // Consumir el cuerpo

    // Obtener el encabezado Content-Type
    let content_type = headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok());

    // Construir la respuesta reenviada
    let mut http_response = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status_code.as_u16()).unwrap(),
    );

    if let Some(content_type) = content_type {
        http_response.content_type(content_type);
    } else {
        http_response.content_type("application/octet-stream");
    }

    Ok(http_response.body(response_body))
}