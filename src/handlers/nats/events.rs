use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TenantModifiedEvent {
    pub tenant_id: String,
}
