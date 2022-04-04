use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Duration, FixedOffset, Local, Offset, TimeZone, Utc};
use chrono_tz::{OffsetComponents, Tz, TzOffset};
use pyo3::{
    exceptions,
    prelude::*,
    pyclass::CompareOp,
    types::{PyDateTime, PyDelta, PyTzInfo},
};

lazy_static! {
    pub(crate) static ref UTC: HybridTz = HybridTz::Timespan(Tz::UTC);
    pub(crate) static ref LOCAL: HybridTz = HybridTz::Offset(Local::now().offset().fix());
    pub(crate) static ref UTC_NOW: DateTime<Utc> = Utc::now();
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum HybridTzOffset {
    FixedOffset(FixedOffset),
    TzOffset(TzOffset),
}

impl Offset for HybridTzOffset {
    fn fix(&self) -> FixedOffset {
        match self {
            HybridTzOffset::FixedOffset(offset) => *offset,
            HybridTzOffset::TzOffset(offset) => offset.fix(),
        }
    }
}

impl Display for HybridTzOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HybridTzOffset::FixedOffset(offset) => offset.fmt(f),
            HybridTzOffset::TzOffset(tz_offset) => tz_offset.fmt(f),
        }
    }
}

impl TimeZone for HybridTz {
    type Offset = HybridTzOffset;

    fn from_offset(offset: &HybridTzOffset) -> Self {
        match offset {
            HybridTzOffset::FixedOffset(offset) => Self::Offset(FixedOffset::from_offset(offset)),
            HybridTzOffset::TzOffset(offset) => Self::Timespan(Tz::from_offset(offset)),
        }
    }

    fn offset_from_local_date(
        &self,
        local: &chrono::NaiveDate,
    ) -> chrono::LocalResult<HybridTzOffset> {
        match self {
            HybridTz::Offset(offset) => offset
                .offset_from_local_date(local)
                .map(HybridTzOffset::FixedOffset),
            HybridTz::Timespan(timespan) => timespan
                .offset_from_local_date(local)
                .map(HybridTzOffset::TzOffset),
        }
    }

    fn offset_from_local_datetime(
        &self,
        local: &chrono::NaiveDateTime,
    ) -> chrono::LocalResult<HybridTzOffset> {
        match self {
            HybridTz::Offset(offset) => offset
                .offset_from_local_datetime(local)
                .map(HybridTzOffset::FixedOffset),
            HybridTz::Timespan(timespan) => timespan
                .offset_from_local_datetime(local)
                .map(HybridTzOffset::TzOffset),
        }
    }

    fn offset_from_utc_date(&self, utc: &chrono::NaiveDate) -> HybridTzOffset {
        match self {
            HybridTz::Offset(offset) => {
                HybridTzOffset::FixedOffset(offset.offset_from_utc_date(utc))
            }
            HybridTz::Timespan(timespan) => {
                HybridTzOffset::TzOffset(timespan.offset_from_utc_date(utc))
            }
        }
    }

    fn offset_from_utc_datetime(&self, utc: &chrono::NaiveDateTime) -> HybridTzOffset {
        match self {
            HybridTz::Offset(offset) => {
                HybridTzOffset::FixedOffset(offset.offset_from_utc_datetime(utc))
            }
            HybridTz::Timespan(timespan) => {
                HybridTzOffset::TzOffset(timespan.offset_from_utc_datetime(utc))
            }
        }
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
        match s {
            "utc" | "UTC" => Ok(*UTC),
            "local" => Ok(*LOCAL),
            _ => {
                if let Ok(timespan) = Tz::from_str(s) {
                    Ok(Self::Timespan(timespan))
                } else {
                    let tmp_datetime = DateTime::parse_from_str(
                        &format!("1970-01-01T00:00:00{s}"),
                        "%Y-%m-%dT%H:%M:%S%z",
                    )
                    .map_err(|_| "unknown timezone")?;
                    Ok(Self::Offset(*tmp_datetime.offset()))
                }
            }
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
        let seconds = match self.tz {
            HybridTz::Offset(offset) => offset.local_minus_utc(),
            HybridTz::Timespan(timespan) => UTC_NOW
                .with_timezone(&timespan)
                .offset()
                .fix()
                .local_minus_utc(),
        };

        PyDelta::new(py, 0, seconds, 0, true).unwrap()
    }

    fn __repr__(&self) -> String {
        format!("<Tz [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.tz.to_string()
    }

    fn __richcmp__(&self, py_tz: PyTz, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => match (self.tz, py_tz.tz) {
                (HybridTz::Offset(l), HybridTz::Offset(r)) => Ok(l == r),
                (HybridTz::Offset(l), HybridTz::Timespan(r)) => {
                    Ok(l == UTC_NOW.with_timezone(&r).offset().fix())
                }
                (HybridTz::Timespan(l), HybridTz::Offset(r)) => {
                    Ok(UTC_NOW.with_timezone(&l).offset().fix() == r)
                }
                (HybridTz::Timespan(l), HybridTz::Timespan(r)) => Ok(l == r),
            },
            CompareOp::Ne => Ok(!(self.__richcmp__(py_tz, CompareOp::Eq)?)),
            _ => Err(exceptions::PyTypeError::new_err(
                "only support op '==' and '!='",
            )),
        }
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
                if let Ok(tz_name) = tz.call_method0("tzname").map(|v| v.extract::<&str>()) {
                    Ok(tz_name?
                        .try_into()
                        .map_err(exceptions::PyValueError::new_err)?)
                } else {
                    let dummy_datetime = PyDateTime::new(tz.py(), 1970, 1, 1, 0, 0, 0, 0, None)?;
                    let offset = tz
                        .call_method1("utcoffset", (dummy_datetime,))?
                        .call_method0("total_seconds")?
                        .extract::<f64>()? as i32;
                    let offset = FixedOffset::east(offset);
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
