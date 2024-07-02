use chrono::{DateTime, Local, NaiveDateTime};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Post {
    pub title: String,
    pub content: String,
    #[serde(deserialize_with = "deserialize_from_str", serialize_with = "serialize_to_str")]
    pub created_at: DateTime<Local>,
}

fn deserialize_from_str<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let naive_datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f")
        .map_err(serde::de::Error::custom)?;
    let utc_datetime = naive_datetime.and_utc();
    Ok(DateTime::from(utc_datetime))
}

fn serialize_to_str<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
    serializer.serialize_str(&s)
}
