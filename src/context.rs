use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
    pub client: Arc<reqwest::Client>,
}

impl Context {
    pub fn new(client: Arc<reqwest::Client>) -> Self {
        Context { client }
    }
}

