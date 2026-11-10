# NATS y contratos de integración

## Consume

| Subject | Payload | Comportamiento |
|---|---|---|
| `tenant.modified` | `{ "tenant_id": "..." }` | Invalida en Redis `cache:tenant:<tenant_id>:*`. |

La suscripción se crea al arrancar el proceso en `src/handlers/nats/listeners.rs`. Si la conexión o la suscripción falla, el listener termina y el HTTP no debe considerarse un sustituto de la invalidación.

## Publica

| Subject | Payload | Respuesta HTTP |
|---|---|---|
| `analytics.events` | JSON de `AnalyticsEvent` | `POST /api/analytics/events` responde `202 Accepted` y publica en una tarea asíncrona. |

`AnalyticsEvent` contiene `event_type`, `metadata`, campos adicionales y `tenant_id` opcional. La implementación actual obtiene `tenant_id` primero de `X-Tenant-Id` y después del body; los consumidores deben acordar la forma canónica del identificador.

## Acuerdos operativos

- `NATS_URL` usa `nats://localhost:4222` por defecto.
- Los payloads son JSON UTF-8.
- `tenant.modified` debe incluir siempre `tenant_id` no vacío.
- La publicación de analítica es best-effort: el hub responde antes de confirmar el publish.
- No se documentan subjects no implementados en este repositorio.
