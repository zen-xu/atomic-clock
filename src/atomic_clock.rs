use std::ops::{Div, Mul};

use chrono::{
    DateTime, Datelike, Duration, Local, LocalResult, NaiveDate, NaiveDateTime, Offset, TimeZone,
    Timelike, Utc,
};
use pyo3::{
    exceptions,
    prelude::*,
    types::{PyDate, PyDateAccess, PyDateTime, PyTime, PyTimeAccess, PyType, PyTzInfo},
};
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
        Tz::build(offset, "UTC".to_string())
    };
}

#[pyclass]
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

    // class methods

    #[classmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(tzinfo = \"local\")")]
    fn now(_cls: &PyType, py: Python, tzinfo: TzInfo) -> PyResult<Self> {
        let now = Local::now();
        let tz = Tz::new(py, tzinfo)?;
        let datetime = tz.from_utc_datetime(&now.naive_utc());
        Ok(Self { datetime, tz })
    }

    #[classmethod]
    fn utcnow(_cls: &PyType) -> PyResult<Self> {
        let now = Utc::now();
        let datetime = (*UTC_TZ).from_utc_datetime(&now.naive_utc());
        Ok(Self {
            datetime,
            tz: (*UTC_TZ).clone(),
        })
    }

    #[classmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"local\"))")]
    #[pyo3(text_signature = "(timestamp, tzinfo = \"local\")")]
    fn fromtimestamp(_cls: &PyType, py: Python, timestamp: f64, tzinfo: TzInfo) -> PyResult<Self> {
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

    #[classmethod]
    #[pyo3(text_signature = "(timestamp)")]
    fn utcfromtimestamp(_cls: &PyType, timestamp: f64) -> PyResult<Self> {
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

    #[classmethod]
    #[args(tzinfo = "None")]
    #[pyo3(text_signature = "(dt, tzinfo = \"None\")")]
    fn fromdatetime(
        _cls: &PyType,
        py: Python,
        dt: &PyDateTime,
        tzinfo: Option<TzInfo>,
    ) -> PyResult<Self> {
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

    #[classmethod]
    #[args(tzinfo = "TzInfo::String(String::from(\"UTC\"))")]
    #[pyo3(text_signature = "(date, tzinfo = \"UTC\")")]
    fn fromdate(_cls: &PyType, py: Python, date: &PyDate, tzinfo: TzInfo) -> PyResult<Self> {
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

    #[classmethod]
    #[pyo3(text_signature = "ordinal")]
    fn fromordinal(_cls: &PyType, ordinal: i64) -> PyResult<Self> {
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

    fn clone(&self) -> Self {
        Clone::clone(self)
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
