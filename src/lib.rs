mod atomic_clock;
mod tz;

#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

use atomic_clock::{get, now, utcnow, AtomicClock};
use tz::Tz;

/// A Python module implemented in Rust.
#[pymodule]
fn _atomic_clock(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AtomicClock>()?;
    m.add_class::<Tz>()?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(now, m)?)?;
    m.add_function(wrap_pyfunction!(utcnow, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
