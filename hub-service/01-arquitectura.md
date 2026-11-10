# Propósito y arquitectura

## Para qué se usa

`hub-service` es un gateway HTTP Actix Web para el frontend. Centraliza el punto de entrada, la verificación del JWT, la propagación del contexto autenticado, el idioma y el reenvío hacia `user`, `tenant`, `asset`, `lang`, `analytics` y `ai`.

No persiste entidades de negocio propias. Redis se usa para invalidar caché por tenant y NATS para recibir eventos de invalidación y publicar analítica.

## Capas actuales

```text
src/main.rs                         arranque, dependencias y rutas
src/handlers/http/*.rs              rutas públicas del gateway
src/handlers/verify_token.rs         validación de Authorization Bearer
src/handlers/forwarding.rs           construcción de URL, headers y proxy HTTP
src/handlers/nats/listeners.rs       consumidor tenant.modified
src/context.rs                       clientes compartidos HTTP, Redis y NATS
```

## Flujo HTTP

1. Actix recibe la petición en el puerto de `HTTP_BIND` (por defecto `localhost:8080`).
2. La ruta decide si necesita verificar el JWT y, cuando aplica, obtiene `Claims`.
3. `forward_request` resuelve `{SERVICE}_SERVICE_URL`, sustituye parámetros de ruta y copia el cuerpo.
4. Los headers de identidad del cliente se eliminan y se reconstruyen desde los claims verificados.
5. Se devuelve al frontend el status, `Content-Type` y cuerpo del servicio destino.

Los errores de conexión con el destino se convierten en `502 Bad Gateway`; los status no exitosos del destino se conservan.

## Dependencias

- `USER_SERVICE_URL`, `TENANT_SERVICE_URL`, `ASSET_SERVICE_URL`, `LANG_SERVICE_URL`, `ANALYTICS_SERVICE_URL` y `AI_SERVICE_URL`.
- Redis mediante `REDIS_URL`.
- NATS mediante `NATS_URL`.
- `SECRET_KEY` para validar tokens.
