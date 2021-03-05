use crate::pyrocord::http::http::HTTPClient;
use crate::pyrocord::http::request::Request;
use crate::pyrocord::http::routes::Route;
use crate::pyrocord::models::gateway;
use crate::pyrocord::utils::asyncio;
use super::state;
use pyo3::prelude::*;

#[pyclass]
pub struct Client {}

impl Client {}

#[pymethods]
impl Client {
    #[new]
    pub fn new(token: String) -> Self {
        state::HTTP.set(HTTPClient::new(token));

        Client {}
    }

    fn gateway(&self) -> PyResult<PyObject> {
        let response = state::HTTP.get().request::<gateway::GatewayPayload>(Request {
            body: None,
            headers: None,
            route: Route::GetGateway,
        });
        asyncio::wait(async {
            let response = response.await;
            match response {
                Ok(response) => {
                    Ok(response)
                }
                Err(error) => {
                    panic!("{:?}", error); // TODO: wrap in python error
                }
            }
        })
    }
}
