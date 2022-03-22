mod atomic_clock;

#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

use atomic_clock::AtomicClock;

/// A Python module implemented in Rust.
#[pymodule]
fn _atomic_clock(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AtomicClock>()?;
    Ok(())
}
