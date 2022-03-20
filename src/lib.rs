use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, FixedOffset, Local, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use pyo3::{exceptions, prelude::*, types::PyType};

lazy_static! {
    static ref LOCAL_OFFSET: FixedOffset = {
        let now = Local::now();
        now.offset().fix()
    };
    static ref UTC_OFFSET: FixedOffset = {
        let now = Utc::now();
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
    text_signature = "(year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0, tz = \"local\")"
)]
struct Spear {
    datetime: DateTime<FixedOffset>,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl Spear {
    #[new]
    #[args(
        hour = "0",
        minute = "0",
        second = "0",
        microsecond = "0",
        tz = "\"local\""
    )]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
        tz: &str,
    ) -> PyResult<Self> {
        let offset = try_get_offset(tz)?;

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

    #[classmethod]
    #[args(tz = "\"local\"")]
    #[pyo3(text_signature = "(tz = \"local\")")]
    fn now(_cls: &PyType, tz: &str) -> PyResult<Self> {
        let now = Local::now();
        let offset = try_get_offset(tz)?;
        let datetime = offset.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime })
    }

    #[classmethod]
    #[args(tz = "None")]
    fn utcnow(_cls: &PyType) -> PyResult<Self> {
        let now = Utc::now();
        let offset = *UTC_OFFSET;
        let datetime = offset.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime })
    }
}

fn try_get_offset(tz: &str) -> PyResult<FixedOffset> {
    if tz.to_lowercase() == "local" {
        Ok(*LOCAL_OFFSET)
    } else if tz.contains(':') {
        let tmp_datetime = format!("1970-01-01T00:00:00{}", tz);
        let tmp_datetime = DateTime::parse_from_rfc3339(&tmp_datetime)
            .map_err(|_e| exceptions::PyValueError::new_err("invalid timezone offset"))?;
        Ok(*tmp_datetime.offset())
    } else {
        let tz = if tz.to_lowercase() == "utc" {
            "UTC"
        } else {
            tz
        };
        let tz = Tz::from_str(tz).map_err(exceptions::PyValueError::new_err)?;
        Ok(tz.ymd(1970, 1, 1).offset().fix())
    }
}
