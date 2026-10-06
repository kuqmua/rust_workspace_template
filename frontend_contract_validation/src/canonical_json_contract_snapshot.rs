pub fn canonical_json_contract_snapshot<Payload>(
    payload: &Payload,
    dynamic_fields: &[crate::json_snapshot_dynamic_field_ref::JsonSnapshotDynamicFieldRef<'_>],
) -> Result<
    crate::json_contract_snapshot::JsonContractSnapshot,
    crate::json_contract_snapshot_error::JsonContractSnapshotError,
>
where
    Payload: serde::Serialize,
{
    let mut normalized = serde_json::to_value(payload).map_err(|_error| {
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
    #[test]
    fn test_snapshot_without_dynamic_fields_preserves_nested_values() {
        let payload = serde_json::json!({
            constants_str::TEST_JSON_REQUEST_ID: constants_str::TEST_JSON_FIRST,
            constants_str::ITEMS: [null, true, 1i32, { constants_str::TEST_JSON_REQUEST_ID: constants_str::TEST_JSON_SECOND }],
        });
        assert!(
            crate::canonical_json_contract_snapshot::canonical_json_contract_snapshot(
                &payload,
                &[]
            )
            .is_ok_and(|snapshot| serde_json::from_str::<serde_json::Value>(
                snapshot.as_ref()
            )
            .is_ok_and(|decoded| decoded == payload))
        );
    }

    #[test]
    fn test_snapshot_preserves_serialization_failure_category() {
        let payload = std::collections::BTreeMap::from([([1u8, 2u8], 3u8)]);
        assert_eq!(
            crate::canonical_json_contract_snapshot::canonical_json_contract_snapshot(
                &payload,
                &[]
            ),
            Err(crate::json_contract_snapshot_error::JsonContractSnapshotError::Serialization),
        );
    }

    #[test]
    fn test_snapshot_size_bound_includes_json_string_delimiters() {
        let maximum = constants_usize::VALUE_1_048_576;
        let fitting = constants_str::X.repeat(maximum - constants_usize::TWO);
        assert!(
            crate::canonical_json_contract_snapshot::canonical_json_contract_snapshot(
                &fitting,
                &[]
            )
            .is_ok_and(|snapshot| snapshot.as_ref().len() == maximum)
        );
        let oversized = constants_str::X.repeat(maximum - constants_usize::ONE);
        assert!(crate::canonical_json_contract_snapshot::canonical_json_contract_snapshot(&oversized, &[])
            .is_err_and(|error| matches!(error,
                crate::json_contract_snapshot_error::JsonContractSnapshotError::TooLong(
                    bounded_types::bounded_string_error::BoundedStringError::AboveMaximum { actual_length, maximum_length }
                ) if actual_length.get() == maximum + constants_usize::ONE && maximum_length.get() == maximum
            )));
    }

    #[test]
    fn test_snapshot_dynamic_keys_replace_each_value_kind_and_preserve_siblings() {
        let payload = serde_json::json!({
            constants_str::TEST_JSON_STATUS: 401i32,
            constants_str::ITEMS: [
                {constants_str::TEST_JSON_REQUEST_ID: null},
                {constants_str::TEST_JSON_REQUEST_ID: 42i32},
                {constants_str::TEST_JSON_REQUEST_ID: [constants_str::TEST_JSON_FIRST]},
                {constants_str::TEST_JSON_REQUEST_ID: {constants_str::TEST_JSON_STATUS: false}},
            ],
        });
        let expected = serde_json::json!({
            constants_str::TEST_JSON_STATUS: 401i32,
            constants_str::ITEMS: [
                {constants_str::TEST_JSON_REQUEST_ID: constants_str::JSON_SNAPSHOT_DYNAMIC_VALUE},
                {constants_str::TEST_JSON_REQUEST_ID: constants_str::JSON_SNAPSHOT_DYNAMIC_VALUE},
                {constants_str::TEST_JSON_REQUEST_ID: constants_str::JSON_SNAPSHOT_DYNAMIC_VALUE},
                {constants_str::TEST_JSON_REQUEST_ID: constants_str::JSON_SNAPSHOT_DYNAMIC_VALUE},
            ],
        });
        let result = crate::canonical_json_contract_snapshot::canonical_json_contract_snapshot(
            &payload,
            &[
                constants_str::TEST_JSON_REQUEST_ID.into(),
                constants_str::TEST_JSON_REQUEST_ID.into(),
            ],
        );
        assert!(result.is_ok_and(|snapshot| {
            serde_json::from_str::<serde_json::Value>(snapshot.as_ref())
                .is_ok_and(|decoded| decoded == expected)
        }));
    }
}
