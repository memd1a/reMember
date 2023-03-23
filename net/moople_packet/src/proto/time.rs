use std::{fmt::Debug, time::Duration};

use chrono::{NaiveDateTime, Utc};

use crate::{NetError, NetResult};

use super::wrapped::{PacketTryWrapped, PacketWrapped};

const FT_UT_OFFSET: i64 = 116444736010800000;
const DEFAULT_TIME: i64 = 150842304000000000;
const ZERO_TIME: i64 = 94354848000000000;
const PERMANENT_TIME: i64 = 150841440000000000;

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
        i64::from_le_bytes(value).try_into()
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

impl PacketTryWrapped for MapleTime {
    type Inner = i64;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn packet_try_from(v: Self::Inner) -> NetResult<Self> {
        Self::try_from(v)
    }
}

#[derive(Debug)]
pub struct Ticks(pub u32);

impl PacketWrapped for Ticks {
    type Inner = u32;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self(v)
    }
}

#[derive(Debug)]
pub struct MapleExpiration(pub Option<MapleTime>);

impl From<Option<NaiveDateTime>> for MapleExpiration {
    fn from(value: Option<NaiveDateTime>) -> Self {
        let v: Option<MapleTime> = value.map(|v| v.into());
        v.into()
    }
}

impl From<Option<MapleTime>> for MapleExpiration {
    fn from(value: Option<MapleTime>) -> Self {
        Self(value)
    }
}

impl MapleExpiration {
    pub fn new(time: MapleTime) -> Self {
        Self(Some(time))
    }

    pub fn never() -> Self {
        Self(None)
    }

    pub fn delay(dur: chrono::Duration) -> Self {
        Self::new((Utc::now() + dur).naive_utc().into())
    }
}

impl PacketWrapped for MapleExpiration {
    type Inner = MapleTime;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0.unwrap_or(MapleTime(0))
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self((v.0 != 0).then_some(v))
    }
}

#[derive(Clone, Copy)]
pub struct DurationMs<T>(pub T);

impl<T: Debug> Debug for DurationMs<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}ms", self.0)
    }
}

impl<T> PacketWrapped for DurationMs<T>
where
    T: Copy,
{
    type Inner = T;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self(v)
    }
}

impl<T> From<Duration> for DurationMs<T>
where
    T: TryFrom<u128>,
    T::Error: Debug,
{
    fn from(value: Duration) -> Self {
        Self(T::try_from(value.as_millis()).expect("Milli conversion"))
    }
}

impl<T> From<DurationMs<T>> for Duration
where
    T: Into<u64>,
{
    fn from(value: DurationMs<T>) -> Self {
        Duration::from_millis(value.0.into())
    }
}

pub type MapleDurationMs16 = DurationMs<u16>;
pub type MapleDurationMs32 = DurationMs<u32>;

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::proto::PacketTryWrapped;

    use super::{MapleDurationMs32, MapleTime};

    #[test]
    fn conv() {
        let _def = MapleTime::maple_default();
    }

    #[test]
    fn dur() {
        const MS: u32 = 100;
        let dur = Duration::from_millis(MS as u64);

        let m_dur: MapleDurationMs32 = dur.into();
        assert_eq!(m_dur.packet_into_inner(), MS);
        assert_eq!(dur, m_dur.into());
    }
}
