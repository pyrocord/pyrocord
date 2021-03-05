mod pyrocord;

use crate::pyrocord::client::client;
use crate::pyrocord::client::client::Client;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pythonize::pythonize;
use serde_json::to_string_pretty;


#[pymodule]
fn init_models(_py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[pymodule]
fn pyrocord(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<client::Client>()?;

    let submod = PyModule::new(py, "models")?;
    init_models(py, submod)?;
    m.add_submodule(submod)?;

    Ok(())
}
