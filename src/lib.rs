mod atomic_clock;
mod tz;

#[macro_use]
extern crate lazy_static;

use pyo3::prelude::*;

use atomic_clock::AtomicClock;
use tz::Tz;

/// A Python module implemented in Rust.
#[pymodule]
fn _atomic_clock(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AtomicClock>()?;
    m.add_class::<Tz>()?;
    Ok(())
}
