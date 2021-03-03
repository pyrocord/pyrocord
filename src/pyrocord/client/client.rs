use pyo3::prelude::*;
use crate::pyrocord::http::http::HTTPClient;

#[pyclass]
pub struct Client {
    pub http: HTTPClient,
}

impl Client {

}

#[pymethods]
impl Client {
    #[new]
    pub fn new(token: String) -> Self {
        Client {
            http: HTTPClient::new(token)
        }
    }
}
