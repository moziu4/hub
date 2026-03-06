use std::env;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use perms::Token;

struct TokenError;

pub fn verify_token(req: HttpRequest) -> Result<(), actix_web::Error>
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
    let token = if token_header.starts_with("Bearer ") {
        &token_header[7..] // Extrae el token después de "Bearer "
    } else {
        return Err(ErrorUnauthorized("El encabezado Authorization debe comenzar con 'Bearer '"));
    };

    // Verifica el token usando `Token::verify`
    Token::verify(secret, token).map_err(|_| {
        eprintln!("Error al verificar el token");
        ErrorUnauthorized("Token inválido o expirado")
    })?;
    
    Ok(())

}