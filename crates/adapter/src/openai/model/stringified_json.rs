use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::String(text) => Ok(text),
        value => serde_json::to_string(&value).map_err(serde::de::Error::custom),
    }
}
