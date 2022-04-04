use std::{
    ops::{Div, Mul},
    vec,
};

use chrono::{
    DateTime, Datelike, Duration, Local, LocalResult, NaiveDate, NaiveDateTime, Offset, TimeZone,
    Timelike, Utc,
};
use pyo3::{
    exceptions,
    prelude::*,
    pyclass::CompareOp,
    types::{PyDate, PyDateAccess, PyDateTime, PyDelta, PyTime, PyTimeAccess, PyTuple, PyTzInfo},
};
use relativedelta::RelativeDelta;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

use crate::hybrid_tz::{HybridTz, PyTz, PyTzLike, UTC, UTC_NOW};

const MIN_ORDINAL: i64 = 1;
const MAX_ORDINAL: i64 = 3652059;

#[pyclass(subclass)]
#[pyo3(
    text_signature = "(year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0, tzinfo = \"utc\")"
)]
#[derive(Clone)]
pub struct AtomicClock {
    datetime: DateTime<HybridTz>,
}

// Constructors
#[pymethods]
impl AtomicClock {
    #[new]
    #[args(
        hour = "0",
        minute = "0",
        second = "0",
        microsecond = "0",
        tzinfo = "PyTzLike::utc()"
    )]
    #[allow(clippy::too_many_arguments)]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
        tzinfo: PyTzLike,
    ) -> PyResult<Self> {
        let tz = tzinfo.try_to_tz()?;

        let datetime =
            tz.ymd_opt(year, month, day)
                .and_hms_micro_opt(hour, minute, second, microsecond);

        if matches!(&datetime, LocalResult::None) {
            return Err(exceptions::PyValueError::new_err("invalid datetime"));
        }

        Ok(Self {
            datetime: datetime.unwrap(),
        })
    }

    #[staticmethod]
    #[args(tzinfo = "PyTzLike::local()")]
    #[pyo3(text_signature = "(tzinfo = \"local\")")]
    fn now(tzinfo: PyTzLike) -> PyResult<Self> {
        let tz = tzinfo.try_to_tz()?;
        let now = Local::now();
        let datetime = tz.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime })
    }

    #[staticmethod]
    pub fn utcnow() -> PyResult<Self> {
        let now = Utc::now();
        let datetime = UTC.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime })
    }

    #[staticmethod]
    #[args(tzinfo = "PyTzLike::local()")]
    #[pyo3(text_signature = "(timestamp, tzinfo = \"local\")")]
    fn fromtimestamp(timestamp: f64, tzinfo: PyTzLike) -> PyResult<Self> {
        let tz = tzinfo.try_to_tz()?;
        let mut timestamp = Decimal::from_f64(timestamp).unwrap();
        if timestamp.scale() > 0 {
            timestamp.set_scale(6).unwrap();
        }
        let secs = timestamp.floor();
        let nsecs = (timestamp - secs).mul(Decimal::from_i64(1_000_000_000).unwrap());
        let datetime = tz.from_utc_datetime(&NaiveDateTime::from_timestamp(
            secs.to_i64().unwrap(),
            nsecs.to_u32().unwrap(),
        ));

        Ok(Self { datetime })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(timestamp)")]
    fn utcfromtimestamp(timestamp: f64) -> PyResult<Self> {
        let mut timestamp = Decimal::from_f64(timestamp).unwrap();
        if timestamp.scale() > 0 {
            timestamp.set_scale(6).unwrap();
        }
        let secs = timestamp.floor();
        let nsecs = (timestamp - secs).mul(Decimal::from_i64(1_000_000_000).unwrap());
        let datetime = UTC.from_utc_datetime(&NaiveDateTime::from_timestamp(
            secs.to_i64().unwrap(),
            nsecs.to_u32().unwrap(),
        ));

        Ok(Self { datetime })
    }

    #[staticmethod]
    #[args(tzinfo = "None")]
    #[pyo3(text_signature = "(dt, tzinfo = \"None\")")]
    fn fromdatetime(dt: &PyDateTime, tzinfo: Option<PyTzLike>) -> PyResult<Self> {
        let tz = {
            if let Some(tzinfo) = tzinfo {
                tzinfo.try_to_tz()?
            } else {
                let tz = dt.getattr("tzinfo")?;
                if let Ok(tz) = tz.extract::<&PyTzInfo>() {
                    PyTzLike::PyTzInfo(tz).try_to_tz()?
                } else {
                    *UTC
                }
            }
        };

        let naive = NaiveDate::from_ymd(dt.get_year(), dt.get_month() as u32, dt.get_day() as u32)
            .and_hms_micro(
                dt.get_hour() as u32,
                dt.get_minute() as u32,
                dt.get_second() as u32,
                dt.get_microsecond(),
            );

        Ok(Self {
            datetime: tz.from_local_datetime(&naive).unwrap(),
        })
    }

    #[staticmethod]
    #[args(tzinfo = "PyTzLike::utc()")]
    #[pyo3(text_signature = "(date, tzinfo = \"UTC\")")]
    fn fromdate(date: &PyDate, tzinfo: PyTzLike) -> PyResult<Self> {
        let tz = tzinfo.try_to_tz()?;
        let naive = NaiveDate::from_ymd(
            date.get_year(),
            date.get_month() as u32,
            date.get_day() as u32,
        )
        .and_hms_micro(0, 0, 0, 0);

        Ok(Self {
            datetime: tz.from_local_datetime(&naive).unwrap(),
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(datetime, fmt, tzinfo=None)")]
    fn strptime(datetime: &str, fmt: &str, tzinfo: Option<PyTzLike>) -> PyResult<Self> {
        use chrono::format::{parse, Parsed, StrftimeItems};

        let mut parsed = Parsed::new();
        parse(&mut parsed, datetime, StrftimeItems::new(fmt))
            .map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;

        // set default values
        parsed.year = parsed.year.or(Some(0));
        parsed.month = parsed.month.or(Some(1));
        parsed.day = parsed.day.or(Some(1));
        if parsed.hour_div_12.is_none() {
            parsed.set_hour(0).unwrap();
        }
        parsed.minute = parsed.minute.or(Some(0));
        parsed.second = parsed.second.or(Some(0));
        parsed.nanosecond = parsed.nanosecond.or(Some(0));
        parsed.offset = parsed.offset.or(Some(0));

        let datetime = parsed
            .to_datetime()
            .map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;

        // get tz
        let tz = {
            if let Some(tzinfo) = tzinfo {
                tzinfo.try_to_tz()?
            } else {
                let offset = datetime.offset();
                HybridTz::Offset(*offset)
            }
        };

        Ok(Self {
            datetime: datetime.with_timezone(&tz),
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "(ordinal)")]
    fn fromordinal(ordinal: i64) -> PyResult<Self> {
        if !matches!(ordinal, MIN_ORDINAL..=MAX_ORDINAL) {
            return Err(exceptions::PyValueError::new_err(format!(
                "ordinal {ordinal} is out of range"
            )));
        }

        let datetime = NaiveDate::from_ymd(1, 1, 1).and_hms(0, 0, 0) + Duration::days(ordinal - 1);
        Ok(Self {
            datetime: UTC.from_utc_datetime(&datetime),
        })
    }

    #[staticmethod]
    #[args(frame, start, end, "*", tz = "None", limit = "None")]
    #[pyo3(text_signature = "(frame, start, end=None, *, tz=None, limit=None)")]
    fn range(
        py: Python,
        frame: Frame,
        start: DateTimeLike,
        end: Option<DateTimeLike>,
        tz: Option<PyTzLike>,
        limit: Option<u64>,
    ) -> PyResult<Py<DatetimeRangeIter>> {
        let start = start.to_atomic_clock()?;
        let end_timestamp = if let Some(end) = end {
            let end = end.to_atomic_clock()?;
            if end.timestamp() < start.timestamp() {
                return Err(exceptions::PyValueError::new_err("end is less than start"));
            }
            end.timestamp()
        } else {
            f64::MAX
        };

        let limit = limit.or(Some(u64::MAX)).unwrap();
        let start = if let Some(tz) = tz {
            AtomicClock::new(
                start.datetime.year(),
                start.datetime.month(),
                start.datetime.day(),
                start.datetime.hour(),
                start.datetime.minute(),
                start.datetime.second(),
                start.datetime.nanosecond() / 1000,
                tz,
            )?
        } else {
            start
        };

        let iter = DatetimeRangeIter {
            generator: DatetimeRangeGenerator::new(start, end_timestamp, frame.duration(), limit),
        };

        Py::new(py, iter)
    }

    #[staticmethod]
    #[args(
        frame,
        start,
        end,
        "*",
        tz = "None",
        limit = "None",
        bounds = "Bounds::StartInclude",
        exact = "false"
    )]
    #[pyo3(
        text_signature = "(frame, start, end, *, tz=None, limit=None, bounds=\"[)\", exact=False)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn span_range(
        py: Python,
        frame: Frame,
        start: DateTimeLike,
        end: DateTimeLike,
        tz: Option<PyTzLike>,
        limit: Option<u64>,
        bounds: Bounds,
        exact: bool,
    ) -> PyResult<Py<DatetimeSpanRangeIter>> {
        let limit = limit.or(Some(u64::MAX)).unwrap();
        let (start, end) = if let Some(tz) = tz {
            (
                start.to_atomic_clock()?.replace(
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(tz.clone()),
                )?,
                end.to_atomic_clock()?.replace(
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(tz),
                )?,
            )
        } else {
            (start.to_atomic_clock()?, end.to_atomic_clock()?)
        };
        let start = start
            .span(frame.clone(), 1, Bounds::StartInclude, exact, 1)?
            .0;

        let generator =
            DatetimeRangeGenerator::new(start, end.timestamp(), frame.clone().duration(), limit);

        let iter = DatetimeSpanRangeIter::new(generator, frame, bounds, exact, end);
        Py::new(py, iter)
    }

    #[staticmethod]
    #[args(
        frame,
        start,
        end,
        "*",
        interval = "1",
        tz = "None",
        limit = "None",
        bounds = "Bounds::StartInclude",
        exact = "false"
    )]
    #[pyo3(
        text_signature = "(frame, start, end, *, interval=1, tz=None, limit=None, bounds=\"[)\", exact=False)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn interval(
        py: Python,
        frame: Frame,
        start: DateTimeLike,
        end: DateTimeLike,
        interval: u64,
        tz: Option<PyTzLike>,
        limit: Option<u64>,
        bounds: Bounds,
        exact: bool,
    ) -> PyResult<Py<DatetimeSpanRangeIter>> {
        if interval < 1 {
            return Err(exceptions::PyValueError::new_err(
                "interval has to be a positive int",
            ));
        }

        let limit = limit.or(Some(u64::MAX)).unwrap();
        let (start, end) = if let Some(tz) = tz {
            (
                start.to_atomic_clock()?.replace(
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(tz.clone()),
                )?,
                end.to_atomic_clock()?.replace(
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(tz),
                )?,
            )
        } else {
            (start.to_atomic_clock()?, end.to_atomic_clock()?)
        };
        let start = start
            .span(frame.clone(), 1, Bounds::StartInclude, exact, 1)?
            .0;

        let generator = DatetimeRangeGenerator::new(
            start,
            end.timestamp(),
            frame.clone().duration() * interval as f64,
            limit,
        );

        let iter = DatetimeSpanRangeIter::new(generator, frame, bounds, exact, end);
        Py::new(py, iter)
    }
}

// Protocols
#[pymethods]
impl AtomicClock {
    fn __repr__(&self) -> String {
        format!("<AtomicClock [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.datetime.to_rfc3339()
    }

    fn __format__(&self, formatstr: &str) -> String {
        self.format(formatstr)
    }

    fn __richcmp__(&self, datetime: DateTimeLike, op: CompareOp) -> PyResult<bool> {
        let left_timestamp = self.timestamp();
        let right_timestamp = match datetime {
            DateTimeLike::AtomicClock(d) => d.timestamp(),
            DateTimeLike::PyDateTime(d) => Self::fromdatetime(d, None).unwrap().timestamp(),
        };
        match op {
            CompareOp::Lt => Ok(left_timestamp < right_timestamp),
            CompareOp::Le => Ok(left_timestamp <= right_timestamp),
            CompareOp::Eq => Ok(left_timestamp == right_timestamp),
            CompareOp::Ne => Ok(left_timestamp != right_timestamp),
            CompareOp::Gt => Ok(left_timestamp > right_timestamp),
            CompareOp::Ge => Ok(left_timestamp >= right_timestamp),
        }
    }

    fn __add__(&self, delta: DeltaLike) -> PyResult<Self> {
        match delta {
            DeltaLike::RelativeDelta(PyRelativeDelta {
                years,
                months,
                days,
                hours,
                minutes,
                seconds,
                microseconds,
                weeks,
                quarters,
                weekday,
            }) => self.shift(
                years,
                months,
                days,
                hours,
                minutes,
                seconds,
                microseconds,
                weeks,
                quarters,
                weekday,
            ),
            DeltaLike::PyDelta(delta) => {
                let seconds = delta.call_method1("total_seconds", ())?.extract::<f64>()?;
                let mut seconds = Decimal::from_f64(seconds).unwrap();
                if seconds.scale() > 0 {
                    seconds.set_scale(6).unwrap();
                }
                let microseconds = seconds.mul(Decimal::from_i64(1_000_000).unwrap());
                self.shift(0, 0, 0, 0, 0, 0, microseconds.to_i64().unwrap(), 0, 0, None)
            }
        }
    }

    fn __radd__(&self, delta: DeltaLike) -> PyResult<Self> {
        self.__add__(delta)
    }

    fn __sub__(&self, py: Python, obj: DateTimeOrDeltaLike) -> PyResult<Py<PyAny>> {
        match obj {
            DateTimeOrDeltaLike::DateTimeLike(datetime) => match datetime {
                DateTimeLike::AtomicClock(datetime) => {
                    let duration = self.datetime - datetime.datetime;
                    let delta =
                        PyDelta::new(py, 0, 0, duration.num_microseconds().unwrap() as i32, true)?;
                    Ok(delta.into())
                }
                DateTimeLike::PyDateTime(datetime) => {
                    let datetime = AtomicClock::fromdatetime(datetime, None)?;
                    let duration = self.datetime - datetime.datetime;
                    let delta =
                        PyDelta::new(py, 0, 0, duration.num_microseconds().unwrap() as i32, true)?;
                    Ok(delta.into())
                }
            },
            DateTimeOrDeltaLike::DeltaLike(delta) => match delta {
                DeltaLike::RelativeDelta(delta) => {
                    let datetime = self.__add__(DeltaLike::RelativeDelta(delta.__neg__()))?;
                    Ok(Py::new(py, datetime)?.to_object(py))
                }
                DeltaLike::PyDelta(delta) => {
                    let seconds = delta.call_method1("total_seconds", ())?.extract::<f64>()?;
                    let mut seconds = Decimal::from_f64(seconds).unwrap();
                    if seconds.scale() > 0 {
                        seconds.set_scale(6).unwrap();
                    }
                    let microseconds = seconds.mul(Decimal::from_i64(1_000_000).unwrap());
                    let datetime = self.shift(
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        -microseconds.to_i64().unwrap(),
                        0,
                        0,
                        None,
                    )?;
                    Ok(Py::new(py, datetime)?.to_object(py))
                }
            },
        }
    }
    fn __rsub__<'p>(&self, py: Python<'p>, datetime: DateTimeLike) -> PyResult<&'p PyDelta> {
        match datetime {
            DateTimeLike::AtomicClock(datetime) => {
                let duration = datetime.datetime - self.datetime;
                PyDelta::new(py, 0, 0, duration.num_microseconds().unwrap() as i32, true)
            }
            DateTimeLike::PyDateTime(datetime) => {
                let datetime = AtomicClock::fromdatetime(datetime, None)?;
                let duration = datetime.datetime - self.datetime;
                PyDelta::new(py, 0, 0, duration.num_microseconds().unwrap() as i32, true)
            }
        }
    }
    fn __hash__(&self) -> i64 {
        self.datetime.timestamp_nanos()
    }
}

// Properties
#[pymethods]
impl AtomicClock {
    #[getter]
    fn year(&self) -> i32 {
        self.datetime.year()
    }

    #[getter]
    fn month(&self) -> u32 {
        self.datetime.month()
    }

    #[getter]
    fn day(&self) -> u32 {
        self.datetime.day()
    }

    #[getter]
    fn hour(&self) -> u32 {
        self.datetime.hour()
    }

    #[getter]
    fn minute(&self) -> u32 {
        self.datetime.minute()
    }

    #[getter]
    fn second(&self) -> u32 {
        self.datetime.second()
    }

    #[getter]
    fn microsecond(&self) -> u32 {
        self.datetime.nanosecond() / 1000
    }

    #[getter]
    fn week(&self) -> u32 {
        self.isocalendar().week()
    }

    #[getter]
    fn quarter(&self) -> u32 {
        (self.month() - 1) / 3 + 1
    }

    #[getter]
    fn tzinfo(&self, py: Python) -> PyResult<Py<PyAny>> {
        let py_tz = PyTz::new(self.datetime.timezone());
        Ok(Py::new(py, py_tz)?.to_object(py))
    }

    #[getter]
    fn datetime<'p>(&self, py: Python<'p>) -> &'p PyDateTime {
        PyDateTime::new(
            py,
            self.datetime.year(),
            self.datetime.month() as u8,
            self.datetime.day() as u8,
            self.datetime.hour() as u8,
            self.datetime.minute() as u8,
            self.datetime.second() as u8,
            self.datetime.nanosecond() / 1000,
            Some(&self.tzinfo(py).unwrap()),
        )
        .unwrap()
    }

    #[getter]
    fn naive<'p>(&self, py: Python<'p>) -> &'p PyDateTime {
        let naive_datetime = self.datetime.naive_utc();
        PyDateTime::new(
            py,
            naive_datetime.year(),
            naive_datetime.month() as u8,
            naive_datetime.day() as u8,
            naive_datetime.hour() as u8,
            naive_datetime.minute() as u8,
            naive_datetime.second() as u8,
            naive_datetime.nanosecond() / 1000,
            None,
        )
        .unwrap()
    }

    #[getter]
    fn int_timestamp(&self) -> i64 {
        self.datetime.timestamp()
    }

    #[getter]
    fn float_timestamp(&self) -> f64 {
        self.timestamp()
    }
}

// Methods
#[pymethods]
impl AtomicClock {
    #[args(bounds = "Bounds::BothExclude")]
    #[pyo3(text_signature = "(start, end, bounds: \"()\")")]
    fn is_between(&self, start: &Self, end: &Self, bounds: Bounds) -> bool {
        bounds.is_between(&self.datetime, &start.datetime, &end.datetime)
    }

    #[args(
        frame,
        "*",
        count = 1,
        bounds = "Bounds::StartInclude",
        exact = "false",
        week_start = "1"
    )]
    #[pyo3(text_signature = "(frame, *, count=1, bounds=\"[)\", exact=False, week_start=0)")]
    fn span(
        &self,
        frame: Frame,
        count: i64,
        bounds: Bounds,
        exact: bool,
        week_start: u32,
    ) -> PyResult<(Self, Self)> {
        if !matches!(week_start, 1..=7) {
            return Err(exceptions::PyValueError::new_err(
                "invalid week_start, valid week_start should be 1..7",
            ));
        }

        let mut floor = if exact {
            self.clone()
        } else {
            match frame {
                Frame::Year => self.replace(
                    None,
                    Some(1),
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                )?,
                Frame::Month => self.replace(
                    None,
                    None,
                    Some(1),
                    Some(0),
                    Some(0),
                    Some(0),
                    Some(0),
                    None,
                )?,
                Frame::Day => {
                    self.replace(None, None, None, Some(0), Some(0), Some(0), Some(0), None)?
                }
                Frame::Hour => {
                    self.replace(None, None, None, None, Some(0), Some(0), Some(0), None)?
                }
                Frame::Minute => {
                    self.replace(None, None, None, None, None, Some(0), Some(0), None)?
                }
                Frame::Second => self.replace(None, None, None, None, None, None, Some(0), None)?,
                Frame::Microsecond => {
                    return Err(exceptions::PyValueError::new_err(
                        "span doesn't support frame `microsecond`",
                    ))
                }
                Frame::Week => {
                    let floor =
                        self.replace(None, None, None, Some(0), Some(0), Some(0), Some(0), None)?;
                    let delta = if week_start > self.isoweekday() { 7 } else { 0 };
                    let days = -(self.isoweekday() as i64 - week_start as i64) - delta;
                    floor.shift(0, 0, days, 0, 0, 0, 0, 0, 0, None)?
                }
                Frame::Quarter => self
                    .replace(
                        None,
                        None,
                        Some(1),
                        Some(0),
                        Some(0),
                        Some(0),
                        Some(0),
                        None,
                    )?
                    .shift(
                        0,
                        -((self.month() - 1) as i64) % 3,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        None,
                    )?,
            }
        };

        let mut ceil = AtomicClock {
            datetime: floor.datetime + frame.duration() * count as f64,
        };

        match bounds {
            Bounds::BothInclude => (),
            Bounds::BothExclude => {
                floor = floor.shift(0, 0, 0, 0, 0, 0, 1, 0, 0, None)?;
                ceil = ceil.shift(0, 0, 0, 0, 0, 0, -1, 0, 0, None)?;
            }
            Bounds::StartInclude => {
                ceil = ceil.shift(0, 0, 0, 0, 0, 0, -1, 0, 0, None)?;
            }
            Bounds::EndInclude => {
                floor = floor.shift(0, 0, 0, 0, 0, 0, 1, 0, 0, None)?;
            }
        }

        Ok((floor, ceil))
    }

    #[pyo3(text_signature = "(frame)")]
    fn floor(&self, frame: Frame) -> PyResult<Self> {
        Ok(self.span(frame, 1, Bounds::StartInclude, false, 1)?.0)
    }

    #[pyo3(text_signature = "(frame)")]
    fn ceil(&self, frame: Frame) -> PyResult<Self> {
        Ok(self.span(frame, 1, Bounds::StartInclude, false, 1)?.1)
    }

    fn timestamp(&self) -> f64 {
        let nan_timestamp = Decimal::from_i64(self.datetime.timestamp_nanos()).unwrap();
        nan_timestamp
            .div(Decimal::from_f64(1e9).unwrap())
            .to_f64()
            .unwrap()
    }

    fn date<'p>(&self, py: Python<'p>) -> &'p PyDate {
        PyDate::new(
            py,
            self.datetime.year(),
            self.datetime.month() as u8,
            self.datetime.day() as u8,
        )
        .unwrap()
    }

    fn time<'p>(&self, py: Python<'p>) -> &'p PyTime {
        PyTime::new(
            py,
            self.datetime.hour() as u8,
            self.datetime.minute() as u8,
            self.datetime.second() as u8,
            self.datetime.nanosecond() / 1000,
            None,
        )
        .unwrap()
    }

    fn timez<'p>(&self, py: Python<'p>) -> &'p PyTime {
        PyTime::new(
            py,
            self.datetime.hour() as u8,
            self.datetime.minute() as u8,
            self.datetime.second() as u8,
            self.datetime.nanosecond() / 1000,
            Some(&self.tzinfo(py).unwrap()),
        )
        .unwrap()
    }

    #[args(tz = "None")]
    #[pyo3(text_signature = "(tz = None)")]
    fn astimezone<'p>(&self, py: Python<'p>, tz: Option<PyTzLike>) -> PyResult<&'p PyDateTime> {
        if let Some(tz) = tz {
            Ok(self.to(tz)?.datetime(py))
        } else {
            Ok(self.datetime(py))
        }
    }

    fn utcoffset<'p>(&self, py: Python<'p>) -> &'p PyDelta {
        let seconds = match self.datetime.timezone() {
            HybridTz::Offset(offset) => offset.local_minus_utc(),
            HybridTz::Timespan(timespan) => UTC_NOW
                .with_timezone(&timespan)
                .offset()
                .fix()
                .local_minus_utc(),
        };

        PyDelta::new(py, 0, seconds, 0, true).unwrap()
    }

    fn dst<'p>(&self, py: Python<'p>) -> &'p PyDelta {
        PyDelta::new(
            py,
            0,
            self.datetime.timezone().dst_offset().num_seconds() as i32,
            0,
            true,
        )
        .unwrap()
    }

    fn timetuple<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        self.datetime(py).call_method("timetuple", (), None)
    }

    fn utctimetuple<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        self.datetime(py).call_method("utctimetuple", (), None)
    }

    fn toordinal(&self) -> i64 {
        let duration = self.datetime.naive_utc() - NaiveDate::from_ymd(1, 1, 1).and_hms(0, 0, 0);
        duration.num_days() + 1
    }

    fn weekday(&self) -> u32 {
        self.datetime.weekday().num_days_from_monday()
    }

    fn isoweekday(&self) -> u32 {
        self.datetime.weekday().num_days_from_sunday()
    }

    fn isocalendar(&self) -> IsoCalendarDate {
        let iso_week = self.datetime.iso_week();
        IsoCalendarDate(vec![
            iso_week.year() as u32,
            iso_week.week(),
            self.isoweekday(),
        ])
    }

    fn ctime(&self) -> String {
        self.datetime.format("%a %b %e %T %Y").to_string()
    }

    fn strftime(&self, format: &str) -> String {
        self.datetime.format(format).to_string()
    }

    fn for_json(&self) -> String {
        self.datetime.format("%Y-%m-%dT%H:%M:%S%.f%Z").to_string()
    }

    #[args(sep = "\"T\"", timespec = "\"auto\"")]
    #[pyo3(text_signature = "(spec = \"T\", timespec = \"auto\")")]
    fn isoformat(&self, sep: &str, timespec: &str) -> PyResult<String> {
        let format = match timespec {
            "auto" | "microseconds" => format!("%Y-%m-%d{sep}%H:%M:%S%.f%:z"),
            "hours" => format!("%Y-%m-%d{sep}%H"),
            "minutes" => format!("%Y-%m-%d{sep}%H:%M"),
            "seconds" => format!("%Y-%m-%d{sep}%H:%M:%S"),
            "milliseconds" => format!("%Y-%m-%d{sep}%H:%M:%S%.3f"),
            _ => return Err(exceptions::PyValueError::new_err("Unknown timespec value")),
        };
        Ok(self.datetime.format(&format).to_string())
    }

    fn clone(&self) -> Self {
        Clone::clone(self)
    }

    #[args("*", year, month, day, hour, minute, second, microsecond, tzinfo)]
    #[pyo3(
        text_signature = "(*, year=None, month=None, day=None, hour=None, minute=None, second=None, microsecond=None, tzinfo=None)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn replace(
        &self,
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
        microsecond: Option<u32>,
        tzinfo: Option<PyTzLike>,
    ) -> PyResult<Self> {
        let mut obj = self.clone();

        if let Some(year) = year {
            obj.datetime = obj
                .datetime
                .with_year(year)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid year"))?;
        }

        if let Some(month) = month {
            obj.datetime = obj
                .datetime
                .with_month(month)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid month"))?;
        }

        if let Some(day) = day {
            obj.datetime = obj
                .datetime
                .with_day(day)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid day"))?;
        }

        if let Some(hour) = hour {
            obj.datetime = obj
                .datetime
                .with_hour(hour)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid hour"))?;
        }

        if let Some(minute) = minute {
            obj.datetime = obj
                .datetime
                .with_minute(minute)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid minute"))?;
        }

        if let Some(second) = second {
            obj.datetime = obj
                .datetime
                .with_second(second)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid second"))?;
        }

        if let Some(microsecond) = microsecond {
            obj.datetime = obj
                .datetime
                .with_nanosecond(microsecond * 1000)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid microsecond"))?;
        }

        if let Some(tzinfo) = tzinfo {
            let tz = tzinfo.try_to_tz()?;
            obj.datetime = obj.datetime.with_timezone(&tz);
        }

        Ok(obj)
    }

    #[args(
        "*",
        years = 0,
        months = 0,
        days = 0,
        hours = 0,
        minutes = 0,
        seconds = 0,
        microseconds = 0,
        weeks = 0,
        quarters = 0,
        weekday = "None"
    )]
    #[pyo3(
        text_signature = "(*, years=0, months=0, days=0, hours=0, minutes=0, seconds=0, microseconds=0, weeks=0, quarters=0, weekday=None)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn shift(
        &self,
        years: i32,
        months: i64,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
        microseconds: i64,
        weeks: i64,
        quarters: i64,
        weekday: Option<u32>,
    ) -> PyResult<Self> {
        let mut obj = self.clone();

        let delta = RelativeDelta::with_years(years)
            .and_months(months + quarters * 3)
            .and_days(days + weeks * 7)
            .and_hours(hours)
            .and_minutes(minutes)
            .and_seconds(seconds)
            .and_nanoseconds(microseconds * 1000)
            .new();

        obj.datetime = obj.datetime + delta;

        if let Some(weekday) = weekday {
            if !matches!(weekday, 0..=6) {
                return Err(exceptions::PyValueError::new_err(
                    "invalid weekday, valid weekday should be 0..6",
                ));
            }

            let current_weekday = obj.datetime.weekday().num_days_from_monday();
            if current_weekday <= weekday {
                obj.datetime = obj.datetime + Duration::days((weekday - current_weekday) as i64)
            } else {
                let jumpdays =
                    (current_weekday - (current_weekday - weekday)) + (6 - current_weekday) + 1;
                obj.datetime = obj.datetime + Duration::days(jumpdays as i64);
            }
        }
        Ok(obj)
    }

    #[pyo3(text_signature = "(tzinfo)")]
    fn to(&self, tzinfo: PyTzLike) -> PyResult<Self> {
        let tz = tzinfo.try_to_tz()?;
        Ok(Self {
            datetime: self.datetime.with_timezone(&tz),
        })
    }

    #[args(fmt = "\"%Y-%m-%d %H:%M:%S%Z\"")]
    #[pyo3(text_signature = "(fmt = \"%Y-%m-%d %H:%M:%S%Z\")")]
    fn format(&self, fmt: &str) -> String {
        self.datetime.format(fmt).to_string()
    }
}

#[pyclass]
struct IsoCalendarDate(Vec<u32>);

#[pymethods]
impl IsoCalendarDate {
    fn __repr__(&self) -> String {
        format!(
            "IsoCalendarDate(year={}, week={}, weekday={})",
            self.0[0], self.0[1], self.0[2]
        )
    }

    #[getter]
    fn year(&self) -> u32 {
        self.0[0]
    }

    #[getter]
    fn week(&self) -> u32 {
        self.0[1]
    }

    #[getter]
    fn weekday(&self) -> u32 {
        self.0[2]
    }

    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<IsoCalendarDateIter>> {
        let iter = IsoCalendarDateIter {
            inner: slf.0.clone().into_iter(),
        };
        Py::new(slf.py(), iter)
    }
}

#[pyclass]
struct IsoCalendarDateIter {
    inner: std::vec::IntoIter<u32>,
}

#[pymethods]
impl IsoCalendarDateIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<u32> {
        slf.inner.next()
    }
}

#[derive(Clone)]
enum Bounds {
    BothInclude,
    BothExclude,
    StartInclude,
    EndInclude,
}

impl FromPyObject<'_> for Bounds {
    fn extract<'source>(ob: &pyo3::PyAny) -> PyResult<Self> {
        let bound = ob.extract::<&str>()?;
        let bound = match bound {
            "[]" => Self::BothInclude,
            "()" => Self::BothExclude,
            "[)" => Self::StartInclude,
            "(]" => Self::EndInclude,
            _ => {
                return Err(exceptions::PyValueError::new_err(
                    "invalid bound, valid bound should be '[]', '()', '[)' and '(]'",
                ))
            }
        };
        Ok(bound)
    }
}

impl Bounds {
    fn is_between(
        &self,
        dt: &DateTime<HybridTz>,
        start: &DateTime<HybridTz>,
        end: &DateTime<HybridTz>,
    ) -> bool {
        match self {
            Self::BothInclude => {
                start.timestamp_nanos() <= dt.timestamp_nanos()
                    && dt.timestamp_nanos() <= end.timestamp_nanos()
            }
            Self::BothExclude => {
                start.timestamp_nanos() < dt.timestamp_nanos()
                    && dt.timestamp_nanos() < end.timestamp_nanos()
            }
            Self::StartInclude => {
                start.timestamp_nanos() <= dt.timestamp_nanos()
                    && dt.timestamp_nanos() < end.timestamp_nanos()
            }
            Self::EndInclude => {
                start.timestamp_nanos() < dt.timestamp_nanos()
                    && dt.timestamp_nanos() <= end.timestamp_nanos()
            }
        }
    }
}

#[pyfunction(tzinfo = "PyTzLike::String(\"local\")")]
#[pyo3(text_signature = "(tzinfo = \"local\")")]
pub(crate) fn now(tzinfo: PyTzLike) -> PyResult<AtomicClock> {
    AtomicClock::now(tzinfo)
}

#[pyfunction]
pub(crate) fn utcnow() -> PyResult<AtomicClock> {
    AtomicClock::utcnow()
}

#[pyfunction(py_args = "*", tzinfo = "None")]
#[pyo3(text_signature = "(*args, tzinfo=None)")]
pub(crate) fn get(py_args: &PyTuple, tzinfo: Option<PyTzLike>) -> PyResult<AtomicClock> {
    let datetime = match py_args.len() {
        0 => AtomicClock::utcnow(),
        1 => {
            let arg = &py_args[0];

            if let Ok(dt) = arg.extract::<AtomicClock>() {
                Ok(dt)
            } else if let Ok(timestamp) = arg.extract::<f64>() {
                AtomicClock::fromtimestamp(timestamp, PyTzLike::utc())
            } else if let Ok(timestamp) = arg.extract::<i64>() {
                AtomicClock::fromtimestamp(timestamp as f64, PyTzLike::utc())
            } else if let Ok(datetime) = arg.extract::<&str>() {
                AtomicClock::strptime(datetime, "%Y-%m-%dT%H:%M:%S%.f%z", None)
                    .or_else(|_| AtomicClock::strptime(datetime, "%Y-%m-%dT%H:%M:%S%.f", None))
                    .or_else(|_| AtomicClock::strptime(datetime, "%Y%m%dT%H%M%S%.f", None))
                    .or_else(|_| AtomicClock::strptime(datetime, "%Y%m%dT%H%M%S%.f%z", None))
            } else if let Ok(tz) = arg.extract::<PyTzLike>() {
                AtomicClock::now(tz)
            } else if let Ok(datetime) = arg.extract::<&PyDateTime>() {
                AtomicClock::fromdatetime(datetime, None)
            } else if let Ok(date) = arg.extract::<&PyDate>() {
                AtomicClock::fromdate(date, PyTzLike::String("UTC"))
            } else if let Ok((year, month, day)) = arg.extract::<(i32, u32, u32)>() {
                AtomicClock::new(year, month, day, 0, 0, 0, 0, PyTzLike::utc())
            } else {
                Err(exceptions::PyValueError::new_err(
                    "failed to parse datetime",
                ))
            }
        }
        2 => {
            let arg1 = &py_args[0];
            let arg2 = &py_args[1];

            if let (Ok(datetime), Ok(tz)) =
                (arg1.extract::<&PyDateTime>(), arg2.extract::<PyTzLike>())
            {
                AtomicClock::fromdatetime(datetime, Some(tz))
            } else if let (Ok(date), Ok(tz)) =
                (arg1.extract::<&PyDate>(), arg2.extract::<PyTzLike>())
            {
                AtomicClock::fromdate(date, tz)
            } else if let (Ok(datetime_str), Ok(fmt_str)) =
                (arg1.extract::<&str>(), arg2.extract::<&str>())
            {
                AtomicClock::strptime(datetime_str, fmt_str, None)
            } else {
                Err(exceptions::PyValueError::new_err(
                    "failed to parse datetime",
                ))
            }
        }
        3..=8 => {
            let year = py_args[0].extract::<i32>()?;
            let mut datetime_args = [0, 0, 0, 0, 0, 0, 0];
            for (idx, arg) in py_args[1..].into_iter().enumerate() {
                datetime_args[idx] = arg.extract::<u32>()?;
                if idx == 5 {
                    break;
                }
            }
            let tz = {
                if py_args.len() == 8 {
                    py_args[7].extract::<PyTzLike>()?
                } else {
                    PyTzLike::utc()
                }
            };

            AtomicClock::new(
                year,
                datetime_args[0],
                datetime_args[1],
                datetime_args[2],
                datetime_args[3],
                datetime_args[4],
                datetime_args[5],
                tz,
            )
        }
        _ => Err(exceptions::PyValueError::new_err("invalid args")),
    }?;

    if let Some(tzinfo) = tzinfo {
        Ok(datetime.to(tzinfo)?)
    } else {
        Ok(datetime)
    }
}

struct DatetimeRangeGenerator {
    current: AtomicClock,
    end_timestamp: f64,
    frame: RelativeDelta,
    limit: u64,
    count: u64,
}

impl DatetimeRangeGenerator {
    fn new(current: AtomicClock, end_timestamp: f64, frame: RelativeDelta, limit: u64) -> Self {
        Self {
            current,
            end_timestamp,
            frame,
            limit,
            count: 0,
        }
    }

    fn next(&mut self) -> Option<AtomicClock> {
        if self.count == self.limit {
            return None;
        }
        let result = self.current.clone();
        if self.current.timestamp() <= self.end_timestamp {
            self.count += 1;
            self.current.datetime = self.current.datetime + self.frame;
            Some(result)
        } else {
            None
        }
    }
}

#[pyclass]
struct DatetimeRangeIter {
    generator: DatetimeRangeGenerator,
}

#[pymethods]
impl DatetimeRangeIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<AtomicClock> {
        slf.generator.next()
    }
}

#[derive(Clone)]
enum Frame {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Microsecond,
    Week,
    Quarter,
}

impl FromPyObject<'_> for Frame {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let frame = ob.extract::<&str>()?;
        let frame = match frame {
            "year" => Self::Year,
            "month" => Self::Month,
            "day" => Self::Day,
            "hour" => Self::Hour,
            "minute" => Self::Minute,
            "second" => Self::Second,
            "microsecond" => Self::Microsecond,
            "week" => Self::Week,
            "quarter" => Self::Quarter,
            _ => return Err(exceptions::PyValueError::new_err("invalid frame")),
        };
        Ok(frame)
    }
}

impl Frame {
    fn duration(self) -> RelativeDelta {
        match self {
            Frame::Year => RelativeDelta::with_years(1).new(),
            Frame::Month => RelativeDelta::with_months(1).new(),
            Frame::Day => RelativeDelta::with_days(1).new(),
            Frame::Hour => RelativeDelta::with_hours(1).new(),
            Frame::Minute => RelativeDelta::with_minutes(1).new(),
            Frame::Second => RelativeDelta::with_seconds(1).new(),
            Frame::Microsecond => RelativeDelta::with_nanoseconds(1000).new(),
            Frame::Week => RelativeDelta::with_days(7).new(),
            Frame::Quarter => RelativeDelta::with_months(3).new(),
        }
    }
}

#[derive(FromPyObject)]
enum DateTimeLike<'p> {
    AtomicClock(AtomicClock),
    PyDateTime(&'p PyDateTime),
}

impl DateTimeLike<'_> {
    fn to_atomic_clock(&self) -> PyResult<AtomicClock> {
        match self {
            DateTimeLike::AtomicClock(dt) => Ok(dt.clone()),
            DateTimeLike::PyDateTime(dt) => AtomicClock::fromdatetime(dt, None),
        }
    }
}

#[pyclass(name = "RelativeDelta")]
#[pyo3(
    text_signature = "(*, years = 0, months = 0, days = 0, hours = 0, minutes = 0, seconds = 0, microseconds = 0, weeks = 0, quarters = 0)"
)]
#[derive(Clone)]
pub struct PyRelativeDelta {
    #[pyo3(get, set)]
    years: i32,
    #[pyo3(get, set)]
    months: i64,
    #[pyo3(get, set)]
    days: i64,
    #[pyo3(get, set)]
    hours: i64,
    #[pyo3(get, set)]
    minutes: i64,
    #[pyo3(get, set)]
    seconds: i64,
    #[pyo3(get, set)]
    microseconds: i64,
    #[pyo3(get, set)]
    weeks: i64,
    #[pyo3(get, set)]
    quarters: i64,
    #[pyo3(get, set)]
    weekday: Option<u32>,
}

#[pymethods]
impl PyRelativeDelta {
    #[new]
    #[args(
        "*",
        years = 0,
        months = 0,
        days = 0,
        hours = 0,
        minutes = 0,
        seconds = 0,
        microseconds = 0,
        weeks = 0,
        quarters = 0,
        weekday = "None"
    )]
    #[allow(clippy::too_many_arguments)]
    fn new(
        years: i32,
        months: i64,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
        microseconds: i64,
        weeks: i64,
        quarters: i64,
        weekday: Option<u32>,
    ) -> PyResult<Self> {
        if !matches!(weekday, Some(0..=6) | None) {
            Err(exceptions::PyValueError::new_err(
                "invalid weekday, valid weekday should be 0..6",
            ))
        } else {
            Ok(Self {
                years,
                months,
                days,
                hours,
                minutes,
                seconds,
                microseconds,
                weeks,
                quarters,
                weekday,
            })
        }
    }

    fn clone(&self) -> Self {
        Clone::clone(self)
    }

    fn __repr__(&self) -> String {
        format!("<RelativeDelta [years={:+}, months={:+}, days={:+}, hours={:+}, minutes={:+}, seconds={:+}, microseconds={:+}, weeks={:+}, quarters={:+}, weekday={:+}]>",
                self.years, self.months, self.days, self.hours, self.minutes, self.seconds, self.microseconds, self.weeks, self.quarters, self.weekday.map_or("None".to_string(), |w| w.to_string()))
    }

    fn __neg__(&self) -> Self {
        Self {
            years: -self.years,
            months: -self.months,
            days: -self.days,
            hours: -self.hours,
            minutes: -self.minutes,
            seconds: -self.seconds,
            microseconds: -self.microseconds,
            weeks: -self.weeks,
            quarters: -self.quarters,
            weekday: self.weekday,
        }
    }
}

#[derive(FromPyObject)]
enum DeltaLike<'p> {
    RelativeDelta(PyRelativeDelta),
    PyDelta(&'p PyDelta),
}

#[derive(FromPyObject)]
enum DateTimeOrDeltaLike<'p> {
    DateTimeLike(DateTimeLike<'p>),
    DeltaLike(DeltaLike<'p>),
}

#[pyclass]
struct DatetimeSpanRangeIter {
    generator: DatetimeRangeGenerator,
    frame: Frame,
    bounds: Bounds,
    exact: bool,
    end: AtomicClock,
}

impl DatetimeSpanRangeIter {
    fn new(
        generator: DatetimeRangeGenerator,
        frame: Frame,
        bounds: Bounds,
        exact: bool,
        end: AtomicClock,
    ) -> Self {
        Self {
            generator,
            frame,
            bounds,
            exact,
            end,
        }
    }
}

#[pymethods]
impl DatetimeSpanRangeIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<(AtomicClock, AtomicClock)> {
        let dt = slf.generator.next()?;

        let (floor, mut ceil) = dt
            .span(slf.frame.clone(), 1, slf.bounds.clone(), slf.exact, 1)
            .unwrap();

        if slf.exact && ceil.timestamp() > slf.end.timestamp() {
            if floor.timestamp() == slf.end.timestamp()
                || floor
                    .shift(0, 0, 0, 0, 0, 0, -1, 0, 0, None)
                    .unwrap()
                    .timestamp()
                    == slf.end.timestamp()
            {
                return None;
            }

            ceil = slf.end.clone();
            if matches!(&slf.bounds, Bounds::BothExclude | Bounds::StartInclude) {
                ceil = ceil.shift(0, 0, 0, 0, 0, 0, -1, 0, 0, None).unwrap()
            }
        }
        Some((floor, ceil))
    }
}
