use std::env;
use actix_web::{HttpRequest};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub permissions: Vec<String>,
    pub role_id: u32,
    pub tenant_id: Option<u32>,
    pub agency_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash, Eq, Copy)]
#[repr(u32)]
pub enum Role
{
    SuperAdmin = 1,
    AgencyOwner = 2,
    AgencyAdmin = 3,
    AgencyMember = 4,
    TenantAdmin = 5,
    Editor = 6,
    Client = 7,
    Guest = 8,
}

impl Role {
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            1 => Some(Role::SuperAdmin),
            2 => Some(Role::AgencyOwner),
            3 => Some(Role::AgencyAdmin),
            4 => Some(Role::AgencyMember),
            5 => Some(Role::TenantAdmin),
            6 => Some(Role::Editor),
            7 => Some(Role::Client),
            8 => Some(Role::Guest),
            _ => None,
        }
    }

    pub fn to_id(&self) -> u32 {
        *self as u32
    }
}

pub async fn verify_token(req: HttpRequest) -> Result<Claims, actix_web::Error>
{
    let secret = env::var("SECRET_KEY").map_err(|_| {
        eprintln!("SECRET_KEY no encontrado");
        ErrorInternalServerError("Error interno: SECRET_KEY no configurado")
    })?;

    let token_header = req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("Falta el token de autorización"))?;


    // Valida si el token tiene el formato correcto
    let token_str = if token_header.starts_with("Bearer ") {
        &token_header[7..] // Extrae el token después de "Bearer "
    } else {
        return Err(ErrorUnauthorized("El encabezado Authorization debe comenzar con 'Bearer '"));
    };

    // Verifica el token usando la librería `perms`
    // Como hemos redefinido Claims localmente para que coincida con lo que el usuario espera,
    // debemos asegurar que la verificación devuelva nuestra estructura Claims.
    let perms_claims = perms::Token::verify(secret, token_str).map_err(|e| {
        eprintln!("Error al verificar el token: {:?}", e);
        ErrorUnauthorized("Token inválido o expirado")
    })?;

    // Convertir de perms::token::Claims a nuestra estructura local Claims
    // Usamos serde_json como puente para la conversión rápida entre estructuras compatibles
    let mut claims_json = serde_json::to_value(perms_claims).map_err(|e| {
        eprintln!("Error al serializar claims de perms: {:?}", e);
        ErrorInternalServerError("Error interno de procesamiento de token")
    })?;

    // Si la librería ya no provee tenant_id o agency_id en Claims,
    // nos aseguramos de que existan como null en el JSON para que la deserialización a Option no falle si los necesitamos.
    if let Some(obj) = claims_json.as_object_mut() {
        if !obj.contains_key("tenant_id") {
            obj.insert("tenant_id".to_string(), serde_json::Value::Null);
        }
        if !obj.contains_key("agency_id") {
            obj.insert("agency_id".to_string(), serde_json::Value::Null);
        }
    }

    let claims: Claims = serde_json::from_value(claims_json).map_err(|e| {
        eprintln!("Error al deserializar a Claims local: {:?}", e);
        ErrorInternalServerError("Error interno: Formato de token incompatible")
    })?;

    // Validación de Tenant: Si el usuario no es SuperAdmin, podrías validar acceso aquí
    // según el host o los claims.
    if let Some(_host) = req.headers().get("host").and_then(|v| v.to_str().ok()) {
         if claims.role_id != Role::SuperAdmin.to_id() {
             // Lógica opcional de validación por tenant/host
             // println!("Validando acceso para rol: {} en host: {}", claims.role_id, host);
         }
    }
    
    Ok(claims)
}

pub async fn has_permission(client: &Client, role_id: &str, permission_id: i32) -> bool {
    // Si es SuperAdmin (ID 1), por lo general tiene todos los permisos.
    // Podrías mantener esta excepción o también consultarla al microservicio.
    if role_id == "1" {
        return true;
    }

    let user_service_url = env::var("USER_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:4000".to_string());
    
    let url = format!("{}/api/auth/validate-permission", user_service_url);

    let payload = serde_json::json!({
        "role_id": role_id.parse::<u32>().unwrap_or(0),
        "permission_id": permission_id
    });

    match client.post(&url).json(&payload).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                // Asumimos que el microservicio de usuarios responde 200 si tiene permiso
                true
            } else {
                false
            }
        },
        Err(err) => {
            eprintln!("Error consultando permisos al servicio de usuarios: {}", err);
            false
        }
    }
}