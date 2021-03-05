use serde::{Deserialize, Serialize};

// Currently used for testing Python <-> Rust interoperability.

#[derive(Serialize, Deserialize)]
pub struct GatewayPayload {
    pub url: String,
}
