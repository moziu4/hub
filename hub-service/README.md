# `hub-service`

Documentación del gateway HTTP del proyecto `hub`. El hub es la Open API que consume el frontend: autentica la petición cuando corresponde, normaliza el contexto de idioma e identidad y reenvía la solicitud al microservicio propietario.

## Lectura rápida

1. [Arquitectura y responsabilidades](01-arquitectura.md)
2. [Entidades y structs de interés](02-dominio.md)
3. [OpenAPI resumida](03-api-openapi.yaml)
4. [NATS y contratos](04-nats.md)
5. [Proxy, headers y configuración](05-proxy-configuracion.md)

La documentación describe el código actual de `hub`; no crea endpoints en los microservicios ni sustituye sus especificaciones locales.
