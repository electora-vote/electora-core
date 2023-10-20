use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(not(target_arch = "wasm32"))]
mod python {
    use super::greet;
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    #[pymodule]
    pub fn electora_core(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(greet_py, m)?)?;
        Ok(())
    }

    #[pyfunction]
    pub fn greet_py(name: &str) -> PyResult<String> {
        Ok(greet(name))
    }
}

