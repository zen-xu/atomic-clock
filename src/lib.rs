mod atomic_clock;
mod hybrid_tz;

#[macro_use]
extern crate lazy_static;

use hybrid_tz::PyTz;
use pyo3::prelude::*;

use atomic_clock::{get, now, utcnow, AtomicClock, PyRelativeDelta};

/// A Python module implemented in Rust.
#[pymodule]
fn atomic_clock(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AtomicClock>()?;
    m.add_class::<PyRelativeDelta>()?;
    m.add_class::<PyTz>()?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(now, m)?)?;
    m.add_function(wrap_pyfunction!(utcnow, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
