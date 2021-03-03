use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

// Currently used for testing Python <-> Rust interoperability.

#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct GatewayPayload {
    url: String,
}
