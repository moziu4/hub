# Proxy, headers y configuración

## Resolución de destinos

Para un servicio lógico `service`, el hub busca `{SERVICE}_SERVICE_URL` en mayúsculas y concatena el endpoint del handler. Si falta la variable, `forwarding.rs` usa `http://localhost:4000`; en despliegue debe configurarse explícitamente cada destino para evitar enviar tráfico al servicio equivocado.

## Headers de entrada y salida

El frontend debe enviar:

```http
Authorization: Bearer <jwt>
X-Language: es
Accept: application/json
```

`X-User-Id` y `X-Tenant-Id` no son una fuente de confianza para identidad. El proxy elimina esos valores recibidos y, cuando la ruta verificó el token, envía los derivados de `Claims`:

```http
X-User-Id: <claims.sub>
X-Tenant-Id: <claims.tenant_id>
X-User-Role: <claims.role_id>
X-User-Organization-Id: <claims.organization_id>
```

El tenant es obligatorio en las rutas de IA. El idioma se normaliza con prioridad `cookie lang`, `X-Language` y `Accept-Language`.

## Variables

| Variable | Uso | Default local |
|---|---|---|
| `HTTP_BIND` | Dirección HTTP del hub | `localhost:8080` |
| `SECRET_KEY` | Verificación de JWT | Obligatoria para rutas autenticadas |
| `REDIS_URL` | Redis | `redis://127.0.0.1/` |
| `NATS_URL` | NATS | `nats://localhost:4222` |
| `*_SERVICE_URL` | URL base de cada microservicio | `http://localhost:4000` si falta |

## CORS

Se permiten `Authorization`, `Content-Type`, `Accept`, `X-Tenant-Id`, `X-User-Id` y `X-Language`; los métodos permitidos son `GET`, `POST`, `PATCH`, `DELETE` y `OPTIONS`.

## Nota de seguridad

Las rutas deben verificar el JWT antes de usar identidad. Las rutas históricas que pasan `None` a `forward_request` no deben interpretarse como autenticadas solo porque el cliente envíe `X-User-Id` o `X-Tenant-Id`; esos headers se eliminan al reenviar.
