use serde::Serialize;

pub trait IntoJson: Serialize {
    fn into_json(&self) -> serde_json::Value
    where
        Self: Serialize,
    {
        serde_json::to_value(self).unwrap()
    }
}
