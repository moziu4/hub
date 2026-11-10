# Entidades y structs de interés

El hub no es propietario del dominio de usuarios, tenants, assets o IA. Sus structs representan contexto de integración.

## `Claims`

Definido en `src/handlers/verify_token.rs`:

- `sub`: identificador del usuario.
- `exp`: expiración del token.
- `permissions`: permisos declarados.
- `role_id`: rol numérico.
- `tenant_id`: tenant autenticado, opcional en el JWT.
- `organization_id`: organización autenticada, opcional.

`verify_token` exige `Authorization: Bearer <token>`, valida el token con `perms` y deserializa estos claims.

## `Context`

`src/context.rs` mantiene clientes compartidos:

- `reqwest::Client` para proxy HTTP.
- `redis::Client` para caché.
- `async_nats::Client` para mensajería.

## `HTTPMethod` y `DataType`

`src/handlers/forwarding.rs` modela los métodos `GET`, `POST`, `PUT`, `PATCH` y `DELETE`, y los cuerpos `JSON`, `Multipart` o `None`.

## Eventos

`TenantModifiedEvent` contiene `tenant_id`. Al recibirlo, el listener busca con `SCAN` `cache:tenant:<tenant_id>:*` y elimina las claves encontradas.

## Propiedad de datos

Los modelos completos y sus reglas viven en cada servicio de `guia-tramite`: `user-service`, `tenant-service`, `asset-service`, `lang-service` y `ai-service`. El frontend debe tratar las respuestas del hub como contratos de esos servicios, no como entidades persistidas por el hub.
