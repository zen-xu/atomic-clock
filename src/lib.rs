use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn spear(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Spear>()?;
    Ok(())
}

#[pyclass]
#[pyo3(text_signature = "(year, month, day, hour = 0, minute = 0, second = 0, microsecond = 0)")]
struct Spear {
    datetime: DateTime<Utc>,
}

#[pymethods]
impl Spear {
    #[new]
    #[args(hour = "0", minute = "0", second = "0", microsecond = "0")]
    fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        microsecond: u32,
        // tzinfo: &str,
    ) -> Self {
        let datetime = DateTime::from_utc(
            NaiveDateTime::new(
                NaiveDate::from_ymd(year, month, day),
                NaiveTime::from_hms_micro(hour, minute, second, microsecond),
            ),
            Utc,
        );
        Self { datetime }
    }

    fn __repr__(&self) -> String {
        format!("<Spear [{}]>", self.__str__())
    }

    fn __str__(&self) -> String {
        self.datetime.to_rfc3339()
    }
}
