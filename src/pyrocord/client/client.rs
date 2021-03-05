use super::controller;
use super::state;
use crate::pyrocord::http::client::HTTPClient;
use crate::pyrocord::utils::asyncio;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass]
pub struct Client {
    controller: Arc<controller::ClientController>,
}

impl Client {}

#[pymethods]
impl Client {
    #[new]
    pub fn new(token: String) -> Self {
        state::HTTP.set(HTTPClient::new(token));

        Client {
            controller: Arc::new(controller::ClientController {}),
        }
    }

    pub fn start(&self) -> PyResult<PyObject> {
        let controller = self.controller.clone();
        asyncio::wait(async move {
            controller.launch().await;
            Ok(true)
        })
    }

    pub fn run(&self) {
        let controller = self.controller.clone();
        asyncio::block(async {
            controller.launch().await;
        })
    }
}
