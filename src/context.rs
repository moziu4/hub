use async_nats::Client as NatsClient;
use redis::Client as RedisClient;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
    pub client: Arc<reqwest::Client>,
    pub redis_client: Arc<RedisClient>,
    pub nats_client: Arc<NatsClient>,
}

impl Context {
    pub fn new(client: Arc<reqwest::Client>, redis_client: Arc<RedisClient>, nats_client: Arc<NatsClient>) -> Self {
        Context { client, redis_client, nats_client }
    }
}
