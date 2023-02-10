use std::fmt::Debug;

use chrono::{NaiveDateTime, Utc};

use crate::{NetError, NetResult};

use super::wrapped::{MapleTryWrapped, MapleWrapped};

const FT_UT_OFFSET: i64 = 116444736010800000;
const DEFAULT_TIME: i64 = 150842304000000000;
const ZERO_TIME: i64 = 94354848000000000;
const PERMANENT_TIME: i64 = 150841440000000000;

fn date_time_from_secs(v: i64) -> NetResult<NaiveDateTime> {
    NaiveDateTime::from_timestamp_millis(v * 1_000).ok_or_else(|| NetError::InvalidTimestamp(v))
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct MapleTime(pub i64);

impl Debug for MapleTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            DEFAULT_TIME => "DEFAULT_TIME".fmt(f),
            ZERO_TIME => "ZERO_TIME".fmt(f),
            PERMANENT_TIME => "PERMANENT_TIME".fmt(f),
            _ => self.as_date_time().fmt(f),
        }
    }
}

impl TryFrom<i64> for MapleTime {
    type Error = NetError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        //TODO check for validity
        Ok(Self(value))
    }
}

impl TryFrom<[u8; 8]> for MapleTime {
    type Error = NetError;

    fn try_from(value: [u8; 8]) -> Result<Self, Self::Error> {
        let v = i64::from_le_bytes(value);
        v.try_into()
    }
}

impl From<NaiveDateTime> for MapleTime {
    fn from(dt: NaiveDateTime) -> Self {
        Self(dt.timestamp_millis() * 10_000 + FT_UT_OFFSET)
    }
}

impl From<MapleTime> for NaiveDateTime {
    fn from(s: MapleTime) -> Self {
        s.as_date_time()
    }
}

impl MapleTime {
    pub fn utc_now() -> Self {
        Self::from(chrono::Utc::now().naive_utc())
    }

    pub fn maple_default() -> Self {
        Self(DEFAULT_TIME)
    }

    pub fn is_maple_default(&self) -> bool {
        self.0 == DEFAULT_TIME
    }

    pub fn zero() -> Self {
        Self(ZERO_TIME)
    }

    pub fn is_zero(&self) -> bool {
        self.0 == ZERO_TIME
    }

    pub fn permanent() -> Self {
        Self(PERMANENT_TIME)
    }

    pub fn is_permanent(&self) -> bool {
        self.0 == PERMANENT_TIME
    }

    pub fn as_date_time(&self) -> NaiveDateTime {
        let n = self.0 - FT_UT_OFFSET;
        NaiveDateTime::from_timestamp_millis(n / 10_000).unwrap()
    }
}

impl MapleTryWrapped for MapleTime {
    type Inner = i64;

    fn maple_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn maple_try_from(v: Self::Inner) -> NetResult<Self> {
        Self::try_from(v)
    }
}

#[derive(Debug, Clone)]
pub struct Timestamp32(pub NaiveDateTime);

impl Timestamp32 {
    pub fn zero() -> Timestamp32 {
        Self(NaiveDateTime::from_timestamp_millis(0).unwrap())
    }

    pub fn now() -> Timestamp32 {
        Self(Utc::now().naive_utc())
    }
}

impl MapleTryWrapped for Timestamp32 {
    type Inner = i32;

    fn maple_into_inner(&self) -> Self::Inner {
        //TODO handle overflow
        self.0.timestamp() as i32
    }

    fn maple_try_from(v: Self::Inner) -> NetResult<Self> {
        let t = date_time_from_secs(v as i64)?;
        Ok(Timestamp32(t))
    }
}

#[derive(Debug, Clone)]
pub struct Timestamp64(pub i64);

impl Timestamp64 {
    pub fn now() -> Self {
        Self::from_date_time(Utc::now().naive_utc())
    }

    pub fn as_date_time(&self) -> NetResult<NaiveDateTime> {
        date_time_from_secs(self.0)
    }

    pub fn from_date_time(dt: NaiveDateTime) -> Self {
        Self(dt.timestamp())
    }
}

impl MapleTryWrapped for Timestamp64 {
    type Inner = i64;

    fn maple_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn maple_try_from(v: Self::Inner) -> NetResult<Self> {
        // ToDo Verifys
        Ok(Self(v))
    }
}

#[derive(Debug)]
pub struct Ticks(pub u32);

impl MapleWrapped for Ticks {
    type Inner = u32;

    fn maple_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self(v)
    }
}
#[cfg(test)]
mod tests {
    use super::MapleTime;

    #[test]
    fn conv() {
        let _def = MapleTime::maple_default();
    }
}
