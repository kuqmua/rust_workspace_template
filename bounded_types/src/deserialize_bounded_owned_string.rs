pub(crate) fn deserialize_bounded_owned_string<'de, Deserializer, Value>(
    deserializer: Deserializer,
) -> Result<Value, Deserializer::Error>
where
    Deserializer: serde::Deserializer<'de>,
    Value: TryFrom<String>,
    Value::Error: std::fmt::Display,
{
    let value = <String as serde::Deserialize>::deserialize(deserializer)?;
    Value::try_from(value).map_err(serde::de::Error::custom)
}
