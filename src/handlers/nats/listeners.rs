use std::sync::Arc;
use futures::StreamExt;
use crate::context::Context;
use super::events::TenantModifiedEvent;

pub async fn start_nats_listener(ctx: Arc<Context>) {
    let mut tenant_sub = match ctx.nats_client.subscribe("tenant.modified").await {
        Ok(sub) => {
            println!("Suscrito a NATS subject 'tenant.modified'");
            sub
        }
        Err(e) => {
            eprintln!("Error suscribiéndose a NATS 'tenant.modified': {}", e);
            return;
        }
    };

    while let Some(message) = tenant_sub.next().await {
        let payload = message.payload;
        if let Ok(event) = serde_json::from_slice::<TenantModifiedEvent>(&payload) {
            println!("Evento NATS tenant.modified recibido: {:?}", event);
            
            let pattern = format!("cache:tenant:{}:*", event.tenant_id);
            
            match ctx.redis_client.get_multiplexed_async_connection().await {
                Ok(mut conn) => {
                    let mut cursor = 0u64;
                    loop {
                        let (new_cursor, keys): (u64, Vec<String>) = match redis::cmd("SCAN")
                            .arg(cursor)
                            .arg("MATCH")
                            .arg(&pattern)
                            .arg("COUNT")
                            .arg(100)
                            .query_async(&mut conn)
                            .await 
                        {
                            Ok(res) => res,
                            Err(e) => {
                                eprintln!("Error ejecutando SCAN en Redis: {}", e);
                                break;
                            }
                        };

                        if !keys.is_empty() {
                            let _: Result<(), _> = redis::cmd("DEL").arg(&keys).query_async(&mut conn).await;
                            println!("Eliminadas {} claves de caché para el tenant: {}", keys.len(), event.tenant_id);
                        }

                        cursor = new_cursor;
                        if cursor == 0 {
                            break;
                        }
                    }
                }
                Err(e) => eprintln!("Error conectando a Redis para invalidar caché masiva: {}", e),
            }
        }
    }
}
