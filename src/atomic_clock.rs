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
    types::{PyDate, PyDateAccess, PyDateTime, PyDelta, PyTime, PyTimeAccess, PyType, PyTzInfo},
};
use relativedelta::RelativeDelta;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};

use crate::tz::{Tz, TzInfo};

const MIN_ORDINAL: i64 = 1;
const MAX_ORDINAL: i64 = 3652059;

lazy_static! {
    static ref UTC_TZ: Tz = {
        let now = Utc::now();
        let offset = now.offset().fix();
        Tz::build(offset, "UTC".to_string(), None)
    };
}

#[pyclass(subclass)]
#[pyo3(
    text_signature = "(year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0, tzinfo = \"local\")"
)]
#[derive(Clone)]
pub struct AtomicClock {
    datetime: DateTime<Tz>,
    tz: Tz,
}

#[pymethods]
impl AtomicClock {
    #[new]
    #[args(
        hour = "0",
        minute = "0",
        second = "0",
        microsecond = "0",
        tzinfo = "TzInfo::String(String::from(\"UTC\"))"
    )]
    #[allow(clippy::too_many_arguments)]
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
        let tz = Tz::new(py, tzinfo)?;
        let datetime =
            tz.ymd_opt(year, month, day)
                .and_hms_micro_opt(hour, minute, second, microsecond);

        if matches!(&datetime, LocalResult::None) {
            return Err(exceptions::PyValueError::new_err("invalid datetime"));
        }

        Ok(Self {
            datetime: datetime.unwrap(),
            tz,
        })
    }

    fn __repr__(&self) -> String {
        format!("<AtomicClock [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.datetime.to_rfc3339()
    }

    fn __format__(&self, formatstr: &str) -> String {
        self.format(formatstr)
    }

    // static methods
    #[staticmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(tzinfo = \"local\")")]
    pub fn now(py: Python, tzinfo: TzInfo) -> PyResult<Self> {
        let now = Local::now();
        let tz = Tz::new(py, tzinfo)?;
        let datetime = tz.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime, tz })
    }

    #[staticmethod]
    pub fn utcnow() -> PyResult<Self> {
        let now = Utc::now();
        let datetime = (*UTC_TZ).from_utc_datetime(&now.naive_utc());
        Ok(Self {
            datetime,
            tz: (*UTC_TZ).clone(),
        })
    }

    #[staticmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(timestamp, tzinfo = \"local\")")]
    fn fromtimestamp(py: Python, timestamp: f64, tzinfo: TzInfo) -> PyResult<Self> {
        let tz = Tz::new(py, tzinfo)?;
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

        Ok(Self { datetime, tz })
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
        let datetime = (*UTC_TZ).from_utc_datetime(&NaiveDateTime::from_timestamp(
            secs.to_i64().unwrap(),
            nsecs.to_u32().unwrap(),
        ));

        Ok(Self {
            datetime,
            tz: (*UTC_TZ).clone(),
        })
    }

    #[staticmethod]
    #[args(tzinfo = "None")]
    #[pyo3(text_signature = "(dt, tzinfo = \"None\")")]
    fn fromdatetime(py: Python, dt: &PyDateTime, tzinfo: Option<TzInfo>) -> PyResult<Self> {
        let tz = {
            let tzinfo = if let Some(tzinfo) = tzinfo {
                tzinfo
            } else {
                let tz = dt.getattr("tzinfo")?;
                if let Ok(tz) = tz.extract::<&PyTzInfo>() {
                    TzInfo::Tz(tz)
                } else {
                    TzInfo::String("UTC".to_string())
                }
            };
            Tz::new(py, tzinfo)?
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
            tz,
        })
    }

    #[staticmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"UTC\"))")]
    #[pyo3(text_signature = "(date, tzinfo = \"UTC\")")]
    fn fromdate(py: Python, date: &PyDate, tzinfo: TzInfo) -> PyResult<Self> {
        let tz = Tz::new(py, tzinfo)?;
        let naive = NaiveDate::from_ymd(
            date.get_year(),
            date.get_month() as u32,
            date.get_day() as u32,
        )
        .and_hms_micro(0, 0, 0, 0);

        Ok(Self {
            datetime: tz.from_utc_datetime(&naive),
            tz,
        })
    }

    #[staticmethod]
    #[pyo3(text_signature = "ordinal")]
    fn strptime(py: Python, date_str: &str, fmt: &str, tzinfo: Option<TzInfo>) -> PyResult<Self> {
        let tzinfo = tzinfo
            .or_else(|| Some(TzInfo::String("UTC".to_string())))
            .unwrap();
        let tz = Tz::new(py, tzinfo)?;
        let datetime = tz
            .datetime_from_str(date_str, fmt)
            .map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { datetime, tz })
    }

    #[staticmethod]
    #[pyo3(text_signature = "ordinal")]
    fn fromordinal(ordinal: i64) -> PyResult<Self> {
        if !matches!(ordinal, MIN_ORDINAL..=MAX_ORDINAL) {
            return Err(exceptions::PyValueError::new_err(format!(
                "ordinal {ordinal} is out of range"
            )));
        }

        let datetime = NaiveDate::from_ymd(1, 1, 1).and_hms(0, 0, 0) + Duration::days(ordinal - 1);
        Ok(Self {
            datetime: (*UTC_TZ).from_utc_datetime(&datetime),
            tz: (*UTC_TZ).clone(),
        })
    }

    // methods
    #[args(bounds = "Bounds::BothExclude")]
    #[pyo3(text_signature = "(start, end, bounds: \"()\")")]
    fn is_between(&self, start: &Self, end: &Self, bounds: Bounds) -> bool {
        bounds.is_between(&self.datetime, &start.datetime, &end.datetime)
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
    fn astimezone<'p>(&self, py: Python<'p>, tz: Option<TzInfo>) -> PyResult<&'p PyDateTime> {
        if let Some(tz) = tz {
            Ok(self.to(py, tz)?.datetime(py))
        } else {
            Ok(self.datetime(py))
        }
    }

    fn utcoffset<'p>(&self, py: Python<'p>) -> Option<&'p PyDelta> {
        let dummy_datetime = PyDateTime::new(py, 1, 1, 1, 1, 1, 1, 1, None).unwrap();
        Some(self.tz.utcoffset(py, dummy_datetime))
    }

    fn dst<'p>(&self, py: Python<'p>) -> Option<&'p PyDelta> {
        let dummy_datetime = PyDateTime::new(py, 1, 1, 1, 1, 1, 1, 1, None).unwrap();
        self.tz.dst(py, Some(dummy_datetime))
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
            "auto" | "microseconds" => format!("%Y-%m-%d{sep}%H:%M:%S%.f%Z"),
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

    #[pyo3(
        text_signature = "(*, year=None, month=None, day=None, hour=None, minute=None, second=None, microsecond=None, tzinfo=None)"
    )]
    #[allow(clippy::too_many_arguments)]
    fn replace(
        &self,
        py: Python,
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        hour: Option<u32>,
        minute: Option<u32>,
        second: Option<u32>,
        microsecond: Option<u32>,
        tzinfo: Option<TzInfo>,
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
                .with_minute(second)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid second"))?;
        }

        if let Some(microsecond) = microsecond {
            obj.datetime = obj
                .datetime
                .with_nanosecond(microsecond * 1000)
                .ok_or_else(|| exceptions::PyValueError::new_err("invalid microsecond"))?;
        }

        if let Some(tzinfo) = tzinfo {
            let tz = Tz::new(py, tzinfo)?;
            obj.datetime = obj.datetime.with_timezone(&tz);
            obj.tz = tz;
        }

        Ok(obj)
    }

    #[args(
        years = 0,
        months = 0,
        days = 0,
        hours = 0,
        minutes = 0,
        seconds = 0,
        microseconds = 0,
        weeks = 0,
        quarters = 0
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
    fn to(&self, py: Python, tzinfo: TzInfo) -> PyResult<Self> {
        let tz = Tz::new(py, tzinfo)?;
        Ok(Self {
            datetime: self.datetime.with_timezone(&tz),
            tz,
        })
    }

    #[args(fmt = "\"%Y-%m-%d %H:%M:%S%Z\"")]
    #[pyo3(text_signature = "(fmt = \"%Y-%m-%d %H:%M:%S%Z\")")]
    fn format(&self, fmt: &str) -> String {
        self.datetime.format(fmt).to_string()
    }

    // properties
    #[getter]
    fn tzinfo(&self, py: Python) -> PyResult<Py<PyAny>> {
        Ok(Py::new(py, self.tz.clone())?.to_object(py))
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
    fn is_between(&self, dt: &DateTime<Tz>, start: &DateTime<Tz>, end: &DateTime<Tz>) -> bool {
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
