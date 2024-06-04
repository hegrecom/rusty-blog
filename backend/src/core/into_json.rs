pub trait IntoJson {
    fn into_json(self) -> serde_json::Value;
}

impl<T> IntoJson for T
where
    T: serde::Serialize,
{
    fn into_json(self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
