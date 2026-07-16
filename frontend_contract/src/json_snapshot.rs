const JSON_CONTRACT_SNAPSHOT_MAX_BYTES: usize = 1_048_576usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonContractSnapshot(String);
impl AsRef<str> for JsonContractSnapshot {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl TryFrom<String> for JsonContractSnapshot {
    type Error = JsonContractSnapshotError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > JSON_CONTRACT_SNAPSHOT_MAX_BYTES {
            Err(JsonContractSnapshotError::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct JsonSnapshotDynamicFieldRef<'value_lt>(&'value_lt str);
impl<'value_lt> From<&'value_lt str> for JsonSnapshotDynamicFieldRef<'value_lt> {
    fn from(value: &'value_lt str) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JsonContractSnapshotError {
    TooLong,
    Serialization,
}
impl std::fmt::Display for JsonContractSnapshotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialization => f.write_str(str_constants::JSON_SNAPSHOT_SERIALIZATION_ERROR),
            Self::TooLong => f.write_str(str_constants::JSON_SNAPSHOT_TOO_LONG_ERROR),
        }
    }
}
impl std::error::Error for JsonContractSnapshotError {}

pub fn canonical_json_contract_snapshot<Payload>(
    value: &Payload,
    dynamic_fields: &[JsonSnapshotDynamicFieldRef<'_>],
) -> Result<JsonContractSnapshot, JsonContractSnapshotError>
where
    Payload: serde::Serialize,
{
    let mut normalized =
        serde_json::to_value(value).map_err(|_error| JsonContractSnapshotError::Serialization)?;
    let mut pending = vec![&mut normalized];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values.iter_mut()),
            serde_json::Value::Object(object) => {
                object.iter_mut().for_each(|(name, field_value)| {
                    if dynamic_fields.iter().any(|field| field.0 == name) {
                        *field_value = serde_json::Value::String(String::from(
                            str_constants::JSON_SNAPSHOT_DYNAMIC_VALUE,
                        ));
                    } else {
                        pending.push(field_value);
                    }
                });
            }
            serde_json::Value::Bool(_)
            | serde_json::Value::Null
            | serde_json::Value::Number(_)
            | serde_json::Value::String(_) => {}
        }
    }
    let text = serde_json::to_string_pretty(&normalized)
        .map_err(|_error| JsonContractSnapshotError::Serialization)?;
    JsonContractSnapshot::try_from(text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn dynamic_fields_are_normalized_recursively() {
        let snapshot = super::canonical_json_contract_snapshot(
            &serde_json::json!({
                str_constants::TEST_JSON_REQUEST_ID: str_constants::TEST_JSON_FIRST,
                str_constants::ITEMS: [{ str_constants::TEST_JSON_REQUEST_ID: str_constants::TEST_JSON_SECOND }],
                str_constants::TEST_JSON_STATUS: 401i32
            }),
            &[str_constants::TEST_JSON_REQUEST_ID.into()],
        )
        .expect("d8ddf580");
        assert!(!snapshot.as_ref().contains(str_constants::TEST_JSON_FIRST));
        assert!(!snapshot.as_ref().contains(str_constants::TEST_JSON_SECOND));
        assert_eq!(
            snapshot
                .as_ref()
                .matches(str_constants::JSON_SNAPSHOT_DYNAMIC_VALUE)
                .count(),
            2usize
        );
    }
}
