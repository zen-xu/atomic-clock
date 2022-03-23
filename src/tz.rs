use std::str::FromStr;

use chrono::{DateTime, FixedOffset, Local, Offset, TimeZone};
use chrono_tz::Tz as CTz;
use pyo3::{
    exceptions,
    prelude::*,
    types::{PyDateTime, PyDelta, PyTzInfo},
};

lazy_static! {
    static ref LOCAL_OFFSET: FixedOffset = {
        let now = Local::now();
        now.offset().fix()
    };
}

#[pyclass(extends=PyTzInfo)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Tz {
    pub offset: FixedOffset,
    name: String,
}

impl Tz {
    pub fn build(offset: FixedOffset, name: String) -> Self {
        Self { offset, name }
    }
}

impl TimeZone for Tz {
    type Offset = FixedOffset;

    fn from_offset(offset: &Self::Offset) -> Self {
        Self {
            offset: *offset,
            name: offset.to_string(),
        }
    }

    fn offset_from_local_date(
        &self,
        _local: &chrono::NaiveDate,
    ) -> chrono::LocalResult<Self::Offset> {
        chrono::LocalResult::Single(self.offset)
    }

    fn offset_from_local_datetime(
        &self,
        _local: &chrono::NaiveDateTime,
    ) -> chrono::LocalResult<Self::Offset> {
        chrono::LocalResult::Single(self.offset)
    }

    fn offset_from_utc_date(&self, _utc: &chrono::NaiveDate) -> Self::Offset {
        self.offset
    }

    fn offset_from_utc_datetime(&self, _utc: &chrono::NaiveDateTime) -> Self::Offset {
        self.offset
    }
}

#[pymethods]
impl Tz {
    #[new]
    pub fn new(py: Python, tzinfo: TzInfo) -> PyResult<Self> {
        let (offset, name) = match tzinfo {
            TzInfo::String(ref tzinfo) => {
                let offset = if tzinfo.to_lowercase() == "local" {
                    *LOCAL_OFFSET
                } else if tzinfo.contains(':') {
                    let tmp_datetime = format!("1970-01-01T00:00:00{}", tzinfo);
                    let tmp_datetime =
                        DateTime::parse_from_rfc3339(&tmp_datetime).map_err(|_e| {
                            exceptions::PyValueError::new_err("invalid timezone offset")
                        })?;
                    *tmp_datetime.offset()
                } else {
                    let tzinfo = if tzinfo.to_lowercase() == "utc" {
                        "UTC"
                    } else {
                        tzinfo
                    };
                    let tz = CTz::from_str(tzinfo).map_err(exceptions::PyValueError::new_err)?;

                    tz.ymd(1970, 1, 1).offset().fix()
                };
                (offset, tzinfo.clone())
            }
            TzInfo::Tz(tzinfo) => {
                let tzinfo = tzinfo.to_object(py);
                let dummy_datetime = PyDateTime::new(py, 1, 1, 1, 1, 1, 1, 1, None)?;
                let offset = tzinfo
                    .call_method(py, "utcoffset", (dummy_datetime,), None)?
                    .getattr(py, "seconds")?;
                let offset: i32 = offset.extract(py)?;
                let offset = FixedOffset::east(offset);
                (offset, offset.to_string())
            }
            TzInfo::AtomicClockTz(tzinfo) => return Ok(tzinfo),
        };
        Ok(Self { offset, name })
    }

    fn __repr__(&self) -> String {
        format!("<Tz [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.name.clone()
    }

    fn tzname(&self, _dt: &PyDateTime) -> Option<String> {
        None
    }

    fn dst<'p>(&self, py: Python<'p>, dt: Option<&'p PyDateTime>) -> Option<&'p PyDelta> {
        dt?;
        Some(PyDelta::new(py, 0, 0, 0, true).unwrap())
    }

    fn utcoffset<'p>(&self, py: Python<'p>, _dt: &'p PyDateTime) -> &'p PyDelta {
        PyDelta::new(py, 0, self.offset.local_minus_utc(), 0, true).unwrap()
    }
}

#[derive(FromPyObject)]
pub enum TzInfo<'p> {
    String(String),
    AtomicClockTz(Tz),
    Tz(&'p PyTzInfo),
}