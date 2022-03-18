use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, FixedOffset, Local, Offset, TimeZone};
use chrono_tz::Tz;
use pyo3::{exceptions, prelude::*};

lazy_static! {
    static ref DEFAULT_OFFSET: FixedOffset = {
        let now = Local::now();
        now.offset().fix()
    };
}

/// A Python module implemented in Rust.
#[pymodule]
fn spear(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Spear>()?;
    Ok(())
}

#[pyclass]
#[pyo3(
    text_signature = "(year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0, tz = None)"
)]
struct Spear {
    datetime: DateTime<FixedOffset>,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl Spear {
    #[new]
    #[args(hour = "0", minute = "0", second = "0", microsecond = "0", tz = "None")]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
        tz: Option<&str>,
    ) -> PyResult<Self> {
        let offset = match tz {
            Some(tz) => {
                let tz = Tz::from_str(tz).map_err(exceptions::PyValueError::new_err)?;
                tz.ymd(1970, 1, 1).offset().fix()
            }
            None => *DEFAULT_OFFSET,
        };

        let datetime =
            offset
                .ymd(year, month, day)
                .and_hms_micro(hour, minute, second, microsecond);

        Ok(Self { datetime })
    }

    fn __repr__(&self) -> String {
        format!("<Spear [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.datetime.to_rfc3339()
    }
}
