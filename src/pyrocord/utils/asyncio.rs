use async_compat::Compat;
use pyo3::prelude::*;
use std::future;

pub fn get_event_loop(py: Python) -> PyResult<PyObject> {
    let asyncio = PyModule::import(py, "asyncio")?;
    let event_loop = asyncio.call0("get_running_loop")?;

    Ok(event_loop.into())
}

pub fn create_future() -> (PyObject, PyObject) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let event_loop = get_event_loop(py).expect("Could not load asyncio.");
    let fut = event_loop
        .call_method0(py, "create_future")
        .expect("Could not create future.");
    (event_loop, fut)
}

pub fn set_fut_result<U: pyo3::IntoPy<pyo3::Py<pyo3::PyAny>>>(event_loop: PyObject, fut: PyObject, res: U) -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let sr = fut.getattr(py, "set_result")?;

    event_loop.call_method1(py, "call_soon_threadsafe", (sr, res))?;

    Ok(())
}

pub fn wait<U: pyo3::IntoPy<pyo3::Py<pyo3::PyAny>>, T: future::Future<Output = PyResult<U>> + Send + 'static>(
    fut: T,
) -> PyResult<PyObject> {
    let (event_loop, future_rx) = create_future();
    let future_tx = future_rx.clone();

    smol::spawn(Compat::new(async {
        let response = fut.await;

        match response {
            Ok(response) => Python::with_gil(|py| {
                if let Err(e) = set_fut_result::<U>(event_loop, future_tx, response) {
                    e.print(py);
                }
            }),
            Err(e) => Python::with_gil(|py| {
                e.print(py);
            }),
        }
    }))
    .detach();

    Ok(future_rx)
}

pub fn block<T: future::Future<Output = ()>>(fut: T) {
    smol::block_on(Compat::new(fut));
}
