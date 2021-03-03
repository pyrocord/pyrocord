mod pyrocord;

use crate::pyrocord::client::client;
use crate::pyrocord::client::client::Client;
use crate::pyrocord::http::request::Request;
use crate::pyrocord::http::routes::Route;
use crate::pyrocord::models::gateway;
use crate::pyrocord::utils::asyncio;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pythonize::pythonize;
use serde_json::to_string_pretty;

#[pyfunction]
fn gateway() -> PyResult<PyObject> {
    asyncio::wait(async {
        let client = Client::new(String::from(""));
        let response = client
            .http
            .request::<gateway::GatewayPayload>(Request {
                body: None,
                headers: None,
                route: Route::GetGateway,
            })
            .await;

        match response {
            Ok(response) => {
                let gil = Python::acquire_gil();
                let py = gil.python();
                Ok(pythonize(py, &response).unwrap())
            }
            Err(error) => {
                panic!("{:?}", error); // TODO: wrap in python error
            }
        }
    })
}

#[pymodule]
fn init_models(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<gateway::GatewayPayload>()?;
    Ok(())
}

/// This module is implemented in Rust.
#[pymodule]
fn pyrocord(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gateway, m)?)?;
    m.add_class::<client::Client>()?;

    let submod = PyModule::new(py, "models")?;
    init_models(py, submod)?;
    m.add_submodule(submod)?;

    Ok(())
}
