use std::ops::Not;

use boolinator::Boolinator;
use chrono::{Duration, Utc};
use serde_json::Value;

use crate::errors::*;

// pub fn build_request(parameters: &BTreeMap<String, String>) -> String {
pub fn build_request(parameters: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>) -> String {
    parameters
        .into_iter()
        .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref()))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn build_request_p<S>(payload: S) -> Result<String>
where
    S: serde::Serialize,
{
    Ok(qs::to_string(&payload)?)
}

pub fn build_signed_request(
    parameters: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    recv_window: u64,
) -> Result<String> {
    let s = IntoIterator::into_iter([
        // Include recvWindow if window > 0
        (recv_window > 0).as_option().map(|_| ("recvWindow", recv_window)),
        // Always include timestamp
        Some(("timestamp", get_timestamp()?)),
    ])
    .flatten()
    .map(|(k, v)| format!("{k}={v}"))
    .chain(
        parameters
            .into_iter()
            .filter(|(k, _)| k.as_ref().is_empty().not())
            .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref())),
    )
    .collect::<Vec<String>>()
    .join("&");

    Ok(s)
}

pub fn build_signed_request_p<S>(payload: S, recv_window: u64) -> Result<String>
where
    S: serde::Serialize,
{
    let query_string = qs::to_string(&payload)?;

    let s = IntoIterator::into_iter([
        // Include recvWindow if window > 0
        (recv_window > 0).as_option().map(|_| ("recvWindow", recv_window)),
        // Always include timestamp
        Some(("timestamp", get_timestamp()?)),
    ])
    .flatten()
    .map(|(k, v)| format!("{k}={v}"))
    .collect::<Vec<String>>()
    .join("&");

    let request = if query_string.is_empty() {
        s
    } else {
        format!("{s}&{query_string}")
    };
    Ok(request)
}

pub fn to_i64(v: &Value) -> i64 {
    // TODO: should this return result?
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    // TODO: should this return result?
    v.as_str().unwrap().parse().unwrap()
}

pub fn get_timestamp() -> Result<u64> { Ok(Utc::now().timestamp_millis() as u64) }

/// Returns a duration in milliseconds for the `days`
pub fn days_millis(days: i64) -> i64 { Duration::days(days).num_milliseconds() }

const TRUE: &str = "TRUE";
const FALSE: &str = "FALSE";

pub fn bool_to_string(b: bool) -> String {
    if b {
        TRUE.to_string()
    } else {
        FALSE.to_string()
    }
}

pub fn bool_to_string_some(b: bool) -> Option<String> { Some(bool_to_string(b)) }

pub mod string_to_decimal {
    use std::fmt;

    use rust_decimal::prelude::*;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: fmt::Display,
            S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToDecimal {
            String(String),
        }

        let StringToDecimal::String(s) = StringToDecimal::deserialize(deserializer)?;
        Decimal::from_str(&s).map_err(de::Error::custom)
    }
}

pub mod u64_or_string {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum U64ToString {
            String(String),
            U64(u64),
        }

        match U64ToString::deserialize(deserializer)? {
            U64ToString::String(s) => Ok(s),
            U64ToString::U64(i) => Ok(i.to_string()),
        }
    }
}
