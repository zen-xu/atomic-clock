use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Duration, FixedOffset, Local, Offset, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz};
use pyo3::{
    exceptions,
    prelude::*,
    types::{PyDateTime, PyDelta, PyTzInfo},
};

lazy_static! {
    pub(crate) static ref UTC: HybridTz = HybridTz::Timespan(Tz::UTC);
    pub(crate) static ref LOCAL: HybridTz = HybridTz::Offset(Local::now().offset().fix());
    static ref UTC_NOW: DateTime<Utc> = Utc::now();
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub(crate) enum HybridTz {
    Offset(FixedOffset),
    Timespan(Tz),
}

impl HybridTz {
    pub fn dst_offset(&self) -> Duration {
        match self {
            HybridTz::Offset(_) => Duration::seconds(0),
            HybridTz::Timespan(timespan) => UTC_NOW.with_timezone(timespan).offset().dst_offset(),
        }
    }
}

impl Offset for HybridTz {
    fn fix(&self) -> FixedOffset {
        match self {
            HybridTz::Offset(tz) => tz.fix(),
            HybridTz::Timespan(tz) => UTC_NOW.with_timezone(tz).offset().fix(),
        }
    }
}

impl TimeZone for HybridTz {
    type Offset = HybridTz;

    fn from_offset(offset: &HybridTz) -> HybridTz {
        *offset
    }

    fn offset_from_local_date(&self, _local: &chrono::NaiveDate) -> chrono::LocalResult<HybridTz> {
        chrono::LocalResult::Single(*self)
    }

    fn offset_from_local_datetime(
        &self,
        _local: &chrono::NaiveDateTime,
    ) -> chrono::LocalResult<HybridTz> {
        chrono::LocalResult::Single(*self)
    }

    fn offset_from_utc_date(&self, _utc: &chrono::NaiveDate) -> HybridTz {
        *self
    }

    fn offset_from_utc_datetime(&self, _utc: &chrono::NaiveDateTime) -> HybridTz {
        *self
    }
}

impl Display for HybridTz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HybridTz::Offset(offset) => offset.fmt(f),
            HybridTz::Timespan(timespan) => timespan.fmt(f),
        }
    }
}

impl FromStr for HybridTz {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(timespan) = Tz::from_str(s) {
            Ok(Self::Timespan(timespan))
        } else {
            let tmp_datetime =
                DateTime::parse_from_str(&format!("1970-01-01T00:00:00{s}"), "%Y-%m-%dT%H:%M:%S%z")
                    .map_err(|_| "unknown timezone")?;
            Ok(Self::Offset(*tmp_datetime.offset()))
        }
    }
}

impl TryFrom<&str> for HybridTz {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        HybridTz::from_str(s)
    }
}

#[pyclass(extends=PyTzInfo, name="Tz")]
#[derive(PartialEq, Eq, Hash, Clone)]
pub(crate) struct PyTz {
    tz: HybridTz,
    dst_offset: Duration,
}

impl PyTz {
    pub fn new(tz: HybridTz) -> Self {
        Self {
            tz,
            dst_offset: tz.dst_offset(),
        }
    }
}

#[pymethods]
impl PyTz {
    #[new]
    fn init(tzinfo: PyTzLike) -> PyResult<Self> {
        let tz = tzinfo.try_to_tz()?;
        Ok(Self::new(tz))
    }

    fn tzname(&self) -> Option<&'static str> {
        match self.tz {
            HybridTz::Offset(_) => None,
            HybridTz::Timespan(tz) => Some(tz.name()),
        }
    }

    fn dst<'p>(&self, py: Python<'p>, dt: Option<&'p PyDateTime>) -> Option<&'p PyDelta> {
        dt?;
        Some(PyDelta::new(py, 0, self.dst_offset.num_seconds() as i32, 0, true).unwrap())
    }

    fn utcoffset<'p>(&self, py: Python<'p>, _dt: &'p PyDateTime) -> &'p PyDelta {
        PyDelta::new(py, 0, self.tz.fix().local_minus_utc(), 0, true).unwrap()
    }

    fn __repr__(&self) -> String {
        format!("<Tz [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.tz.to_string()
    }
}

#[derive(FromPyObject, Clone)]
pub(crate) enum PyTzLike<'p> {
    String(&'p str),
    PyTz(PyTz),
    PyTzInfo(&'p PyTzInfo),
}

impl<'p> PyTzLike<'p> {
    pub fn try_to_tz(self) -> PyResult<HybridTz> {
        match self {
            PyTzLike::String(tz) => tz.try_into().map_err(exceptions::PyValueError::new_err),
            PyTzLike::PyTz(tz) => Ok(tz.tz),
            PyTzLike::PyTzInfo(tz) => {
                if let Some(tz_name) = tz.call_method0("tzname")?.extract::<Option<&str>>()? {
                    Ok(tz_name
                        .try_into()
                        .map_err(exceptions::PyValueError::new_err)?)
                } else {
                    let dummy_datetime = PyDateTime::new(tz.py(), 1, 1, 1, 0, 0, 0, 0, None)?;
                    let offset = tz
                        .call_method("utcoffset", (dummy_datetime,), None)?
                        .getattr("seconds")?;
                    let offset = FixedOffset::east(offset.extract::<i32>()?);
                    Ok(HybridTz::Offset(offset))
                }
            }
        }
    }

    pub fn utc() -> Self {
        PyTzLike::PyTz(PyTz::new(*UTC))
    }

    pub fn local() -> Self {
        PyTzLike::PyTz(PyTz::new(*LOCAL))
    }
}
