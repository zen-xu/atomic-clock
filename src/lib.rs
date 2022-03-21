use std::{ops::Mul, str::FromStr};

#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, FixedOffset, Local, LocalResult, NaiveDateTime, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use pyo3::{
    exceptions,
    prelude::*,
    types::{PyDateTime, PyType, PyTzInfo},
};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
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
fn _atomic_clock(_py: Python, m: &PyModule) -> PyResult<()> {
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
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(tzinfo = \"local\")")]
    fn now(_cls: &PyType, py: Python, tzinfo: TzInfo) -> PyResult<Self> {
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

    #[classmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(timestamp, tzinfo = \"local\")")]
    fn fromtimestamp(_cls: &PyType, py: Python, timestamp: f64, tzinfo: TzInfo) -> PyResult<Self> {
        let offset = tzinfo.try_get_offset(py)?;
        let mut timestamp = Decimal::from_f64(timestamp).unwrap();
        if timestamp.scale() > 0 {
            timestamp.set_scale(6).unwrap();
        }
        let secs = timestamp.floor();
        let nsecs = (timestamp - secs).mul(Decimal::from_i64(1_000_000_000).unwrap());
        let datetime = offset.from_utc_datetime(&NaiveDateTime::from_timestamp(
            secs.to_i64().unwrap(),
            nsecs.to_u32().unwrap(),
        ));

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
