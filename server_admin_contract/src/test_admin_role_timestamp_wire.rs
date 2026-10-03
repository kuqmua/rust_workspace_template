#[test]
fn test_role_timestamp_wire_preserves_fractional_second_precision() {
    let optional_date = constants_str::VALUE_2026_07_13T12_30_00
        .split_once('T')
        .map(|(date, _)| date);
    assert!(optional_date.is_some_and(|date| {
        [
            (0u64, None),
            (1u64, Some(stringify!(000001))),
            (10u64, Some(stringify!(00001))),
            (100_000u64, Some(stringify!(1))),
            (999_999u64, Some(stringify!(999999))),
        ]
        .into_iter()
        .all(|(microsecond, fraction)| {
            let wire = serde_json::json!({
                (constants_str::DATE_NAIVE): date,
                (constants_str::PG_CRUD_PG_TIME): {
                    (constants_str::HOUR): 12u64,
                    (constants_str::MIN): 30u64,
                    (constants_str::SEC): 0u64,
                    (constants_str::MICRO): microsecond,
                },
            });
            let expected = fraction.map_or_else(
                || constants_str::VALUE_2026_07_13T12_30_00.to_owned(),
                |digits| format!("{}.{digits}", constants_str::VALUE_2026_07_13T12_30_00),
            );
            serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(wire)
                .is_ok_and(|timestamp| timestamp.to_string() == expected)
        })
    }));
}

#[test]
fn test_role_timestamp_wire_rejects_missing_and_mistyped_components() {
    let optional_date = constants_str::VALUE_2026_07_13T12_30_00
        .split_once('T')
        .map(|(date, _)| date);
    assert!(optional_date.is_some_and(|date| {
        let valid = serde_json::json!({
            (constants_str::DATE_NAIVE): date,
            (constants_str::PG_CRUD_PG_TIME): {
                (constants_str::HOUR): 12u64,
                (constants_str::MIN): 30u64,
                (constants_str::SEC): 0u64,
                (constants_str::MICRO): 0u64,
            },
        });
        let invalid_leaf_values = [
            serde_json::Value::Null,
            serde_json::json!(-1i64),
            serde_json::json!(1.5f64),
            serde_json::json!(constants_str::X),
        ];
        let fields = [
            constants_str::HOUR,
            constants_str::MIN,
            constants_str::SEC,
            constants_str::MICRO,
        ];
        fields.into_iter().all(|field| {
            let mut missing = valid.clone();
            let removed = missing
                .get_mut(constants_str::PG_CRUD_PG_TIME)
                .and_then(serde_json::Value::as_object_mut)
                .and_then(|time| time.remove(field));
            removed.is_some()
                && serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(
                    missing,
                )
                .is_err()
                && invalid_leaf_values.iter().all(|invalid| {
                    let mut wire = valid.clone();
                    let changed = wire
                        .get_mut(constants_str::PG_CRUD_PG_TIME)
                        .and_then(|time| time.get_mut(field))
                        .is_some_and(|component| {
                            *component = invalid.clone();
                            true
                        });
                    changed
                    && serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(
                        wire,
                    )
                    .is_err()
                })
        }) && [constants_str::DATE_NAIVE, constants_str::PG_CRUD_PG_TIME]
            .into_iter()
            .all(|field| {
                let mut missing = valid.clone();
                let removed = missing
                    .as_object_mut()
                    .and_then(|object| object.remove(field));
                let mut mistyped = valid.clone();
                let changed = mistyped.get_mut(field).is_some_and(|component| {
                    *component = serde_json::Value::Null;
                    true
                });
                removed.is_some()
                    && changed
                    && serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(
                        missing,
                    )
                    .is_err()
                    && serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(
                        mistyped,
                    )
                    .is_err()
            })
            && [
                serde_json::Value::Null,
                serde_json::json!([]),
                serde_json::json!(true),
            ]
            .into_iter()
            .all(|wire| {
                serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(wire)
                    .is_err()
            })
    }));
}

#[test]
fn test_role_timestamp_string_deserialization_preserves_character_boundaries() {
    let accepted = [
        String::new(),
        constants_str::X.repeat(64usize),
        '\u{00e9}'.to_string().repeat(64usize),
    ];
    assert!(accepted.into_iter().all(|text| {
        serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(
            serde_json::json!(&text),
        )
        .is_ok_and(|timestamp| {
            timestamp.to_string() == text
                && serde_json::to_value(timestamp).is_ok_and(|wire| wire == serde_json::json!(text))
        })
    }));
    let oversized = [
        constants_str::X.repeat(65usize),
        '\u{00e9}'.to_string().repeat(65usize),
    ];
    assert!(oversized.into_iter().all(|text| {
        let expected = crate::admin_role_timestamp::AdminRoleTimestamp::try_from(text.clone());
        let observed = serde_json::from_value::<crate::admin_role_timestamp::AdminRoleTimestamp>(serde_json::json!(text));
        matches!((expected, observed), (Err(expected_error), Err(observed_error)) if observed_error.to_string().contains(&expected_error.to_string()))
    }));
    let malformed =
        serde_json::from_str::<crate::admin_role_timestamp::AdminRoleTimestamp>(constants_str::X);
    assert!(malformed.is_err_and(|error| error.classify() == serde_json::error::Category::Syntax));
}
