#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = JsonContractSnapshot::validate)]
pub struct JsonContractSnapshot(String);
impl JsonContractSnapshot {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    const fn validate(value: &str) -> Result<(), JsonContractSnapshotError> {
        if value.len() > constants_usize::VALUE_1_048_576 {
            Err(JsonContractSnapshotError::TooLong)
        } else {
            Ok(())
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct JsonSnapshotDynamicFieldRef<'value_lt>(&'value_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum JsonContractSnapshotError {
    #[error("{}", constants_str::JSON_SNAPSHOT_SERIALIZATION_ERROR)]
    Serialization,
    #[error("{}", constants_str::JSON_SNAPSHOT_TOO_LONG_ERROR)]
    TooLong,
}

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
                            constants_str::JSON_SNAPSHOT_DYNAMIC_VALUE,
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
                constants_str::TEST_JSON_REQUEST_ID: constants_str::TEST_JSON_FIRST,
                constants_str::ITEMS: [{ constants_str::TEST_JSON_REQUEST_ID: constants_str::TEST_JSON_SECOND }],
                constants_str::TEST_JSON_STATUS: 401i32
            }),
            &[constants_str::TEST_JSON_REQUEST_ID.into()],
        )
        .expect("d8ddf580 dynamic_fields_are_normalized_recursively invariant must hold");
        assert!(!snapshot.as_ref().contains(constants_str::TEST_JSON_FIRST));
        assert!(!snapshot.as_ref().contains(constants_str::TEST_JSON_SECOND));
        assert_eq!(
            snapshot
                .as_ref()
                .matches(constants_str::JSON_SNAPSHOT_DYNAMIC_VALUE)
                .count(),
            2usize
        );
    }
}
