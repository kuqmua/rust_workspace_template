pub fn canonical_json_contract_snapshot<Payload>(
    value: &Payload,
    dynamic_fields: &[crate::json_snapshot_dynamic_field_ref::JsonSnapshotDynamicFieldRef<'_>],
) -> Result<
    crate::json_contract_snapshot::JsonContractSnapshot,
    crate::json_contract_snapshot_error::JsonContractSnapshotError,
>
where
    Payload: serde::Serialize,
{
    let mut normalized = serde_json::to_value(value).map_err(|_error| {
        crate::json_contract_snapshot_error::JsonContractSnapshotError::Serialization
    })?;
    let mut pending = vec![&mut normalized];
    while let Some(current) = pending.pop() {
        match current {
            serde_json::Value::Array(values) => pending.extend(values.iter_mut()),
            serde_json::Value::Object(object) => {
                object.iter_mut().for_each(|(name, field_value)| {
                    if dynamic_fields.iter().any(|field| field.get() == name) {
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
    let text = serde_json::to_string_pretty(&normalized).map_err(|_error| {
        crate::json_contract_snapshot_error::JsonContractSnapshotError::Serialization
    })?;
    crate::json_contract_snapshot::JsonContractSnapshot::try_from(text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dynamic_fields_are_normalized_recursively() {
        let snapshot = crate::canonical_json_contract_snapshot::canonical_json_contract_snapshot(
            &serde_json::json!({
                constants_str::TEST_JSON_REQUEST_ID: constants_str::TEST_JSON_FIRST,
                constants_str::ITEMS: [{ constants_str::TEST_JSON_REQUEST_ID: constants_str::TEST_JSON_SECOND }],
                constants_str::TEST_JSON_STATUS: 401i32
            }),
            &[constants_str::TEST_JSON_REQUEST_ID.into()],
        )
        .expect(constants_str::DIAGNOSTIC_D8DDF580);
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
