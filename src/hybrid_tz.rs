use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, FixedOffset, Offset, TimeZone, Utc};
use chrono_tz::Tz as CTz;

lazy_static! {
    static ref UTC_NOW: DateTime<Utc> = Utc::now();
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
pub(crate) enum HybridTz {
    Offset(FixedOffset),
    Timespan(CTz),
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
        if let Ok(timespan) = CTz::from_str(s) {
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
