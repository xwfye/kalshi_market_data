use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serializer};

pub type Timestamp = u64;

pub fn from_iso8601_to_timestamp_nanos<'de, T>(deserializer: T) -> Result<Timestamp, T::Error>
where
    T: Deserializer<'de>,
{
    let datetime_string: &str = Deserialize::deserialize(deserializer)?;
    Ok(datetime_string
        .parse::<DateTime<Utc>>()
        .unwrap()
        .timestamp_nanos() as Timestamp)
}

pub fn from_timestamp_nanos_to_iso8601<T>(
    timestamp_nanos: &Timestamp,
    s: T,
) -> Result<T::Ok, T::Error>
where
    T: Serializer,
{
    let secs: i64 = *timestamp_nanos as i64 / 1_000_000_000;
    let nsecs: u32 = (*timestamp_nanos % 1_000_000_000) as u32;
    let ts_datetime: DateTime<Utc> =
        DateTime::from_utc(NaiveDateTime::from_timestamp_opt(secs, nsecs).unwrap(), Utc);
    let iso8601 = ts_datetime.to_rfc3339();
    s.serialize_str(&iso8601)
}

pub fn from_timestamp_millis_to_timestamp_nanos<'de, T>(
    deserializer: T,
) -> Result<Timestamp, T::Error>
where
    T: Deserializer<'de>,
{
    let timestamp_millis: u64 = Deserialize::deserialize(deserializer)?;
    Ok(timestamp_millis * 1_000_000 as Timestamp)
}

pub fn from_timestamp_nanos_to_timestamp_millis<T>(
    timestamp_nanos: &Timestamp,
    s: T,
) -> Result<T::Ok, T::Error>
where
    T: Serializer,
{
    let timestamp_millis = timestamp_nanos / 1_000_000;
    s.serialize_u64(timestamp_millis)
}

pub fn from_timestamp_seconds_to_timestamp_nanos<'de, T>(
    deserializer: T,
) -> Result<Timestamp, T::Error>
where
    T: Deserializer<'de>,
{
    let timestamp_seconds: u64 = Deserialize::deserialize(deserializer)?;
    Ok(timestamp_seconds * 1_000_000_000 as Timestamp)
}

pub fn from_timestamp_nanos_to_timestamp_seconds<T>(
    timestamp_nanos: &Timestamp,
    s: T,
) -> Result<T::Ok, T::Error>
where
    T: Serializer,
{
    let timestamp_seconds = timestamp_nanos / 1_000_000_000;
    s.serialize_u64(timestamp_seconds)
}

pub fn from_string_to_authorization_token<'de, T>(deserializer: T) -> Result<String, T::Error>
where
    T: Deserializer<'de>,
{
    let token: String = Deserialize::deserialize(deserializer)?;
    Ok("Bearer ".to_string() + &token)
}
