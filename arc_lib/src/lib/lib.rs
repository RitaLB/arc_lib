pub mod find_arcos_v1;
pub mod find_arcos_v2;
pub mod find_arcos_v2_antigo;
pub mod ler_pla_antigo;
pub mod ler_pla;
pub mod find_path;
pub mod python_integration;
pub mod write_simulation_file;

use pyo3::prelude::*;

#[pymodule]
pub fn py_arc_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(python_integration::sum_as_string, m)?)?;
    Ok(())
}
