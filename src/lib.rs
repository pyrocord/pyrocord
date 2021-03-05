mod pyrocord;

use crate::pyrocord::client::client;
use pyo3::prelude::*;

#[pymodule]
fn init_models(_py: Python, _m: &PyModule) -> PyResult<()> {
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
