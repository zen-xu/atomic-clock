use std::str::FromStr;

#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, FixedOffset, Local, LocalResult, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use pyo3::{
    exceptions,
    prelude::*,
    types::{PyDateTime, PyType, PyTzInfo},
};

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
fn atomic_clock(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AtomicClock>()?;
    Ok(())
}

#[pyclass]
#[pyo3(
    text_signature = "(year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0, tzinfo = \"local\")"
)]
struct AtomicClock {
    datetime: DateTime<FixedOffset>,
}

#[pymethods]
#[allow(clippy::too_many_arguments)]
impl AtomicClock {
    #[new]
    #[args(
        hour = "0",
        minute = "0",
        second = "0",
        microsecond = "0",
        tzinfo = "TzInfo::String(String::from(\"UTC\"))"
    )]
    fn new(
        py: Python,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
        tzinfo: TzInfo,
    ) -> PyResult<Self> {
        let offset = tzinfo.try_get_offset(py)?;
        let datetime =
            offset
                .ymd_opt(year, month, day)
                .and_hms_micro_opt(hour, minute, second, microsecond);

        if matches!(&datetime, LocalResult::None) {
            return Err(exceptions::PyValueError::new_err("invalid datetime"));
        }

        Ok(Self {
            datetime: datetime.unwrap(),
        })
    }

    fn __repr__(&self) -> String {
        format!("<AtomicClock [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.datetime.to_rfc3339()
    }

    #[classmethod]
    fn demo(_cls: &PyType, py: Python, tzinfo: Option<&PyTzInfo>) -> PyResult<PyObject> {
        let tzinfo = tzinfo.unwrap().to_object(py);
        let datetime = PyDateTime::new(py, 1, 1, 1, 1, 1, 1, 1, None)?;
        tzinfo
            .call_method(py, "utcoffset", (datetime,), None)?
            .getattr(py, "seconds")
    }

    #[classmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(tzinfo = \"local\")")]
    fn now<'p>(_cls: &PyType, py: Python<'p>, tzinfo: TzInfo) -> PyResult<Self> {
        let now = Local::now();
        let offset = tzinfo.try_get_offset(py)?;
        let datetime = offset.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime })
    }

    #[classmethod]
    fn utcnow(_cls: &PyType) -> PyResult<Self> {
        let now = Utc::now();
        let offset = *UTC_OFFSET;
        let datetime = offset.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime })
    }
}

#[derive(FromPyObject)]
enum TzInfo<'p> {
    String(String),
    Tz(&'p PyTzInfo),
}

impl TzInfo<'_> {
    fn try_get_offset(self, py: Python) -> PyResult<FixedOffset> {
        match self {
            Self::String(ref tzinfo) => {
                if tzinfo.to_lowercase() == "local" {
                    Ok(*LOCAL_OFFSET)
                } else if tzinfo.contains(':') {
                    let tmp_datetime = format!("1970-01-01T00:00:00{}", tzinfo);
                    let tmp_datetime =
                        DateTime::parse_from_rfc3339(&tmp_datetime).map_err(|_e| {
                            exceptions::PyValueError::new_err("invalid timezone offset")
                        })?;
                    Ok(*tmp_datetime.offset())
                } else {
                    let tzinfo = if tzinfo.to_lowercase() == "utc" {
                        "UTC"
                    } else {
                        tzinfo
                    };
                    let tz = Tz::from_str(tzinfo).map_err(exceptions::PyValueError::new_err)?;
                    Ok(tz.ymd(1970, 1, 1).offset().fix())
                }
            }
            Self::Tz(tzinfo) => {
                let tzinfo = tzinfo.to_object(py);
                let dummy_datetime = PyDateTime::new(py, 1, 1, 1, 1, 1, 1, 1, None)?;
                let offset = tzinfo
                    .call_method(py, "utcoffset", (dummy_datetime,), None)?
                    .getattr(py, "seconds")?;
                let offset: i32 = offset.extract(py)?;
                Ok(FixedOffset::east(offset))
            }
        }
    }
}
