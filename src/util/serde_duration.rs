use chrono::Duration;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(dt: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.num_seconds().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds = i64::deserialize(deserializer)?;
    Ok(Duration::seconds(seconds))
}
