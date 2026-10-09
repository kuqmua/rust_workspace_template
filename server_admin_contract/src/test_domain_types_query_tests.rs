#[test]
fn test_page_limit_accepts_only_the_contract_range() {
    assert!(matches!(
        crate::admin_page_limit::AdminPageLimit::try_from(
            crate::admin_page_limit::AdminPageLimit::MIN
        ),
        Ok(_value)
    ));
    assert!(matches!(
        crate::admin_page_limit::AdminPageLimit::try_from(constants_u16::ZERO),
        Err(crate::admin_page_limit_error::AdminPageLimitError::OutOfRange)
    ));
    assert!(matches!(
        crate::admin_page_limit::AdminPageLimit::try_from(
            crate::admin_page_limit::AdminPageLimit::MAX.saturating_add(1u16)
        ),
        Err(crate::admin_page_limit_error::AdminPageLimitError::OutOfRange)
    ));
}

#[test]
fn test_pagination_values_deserialize_from_url_query_strings() {
    let Err(_zero_error) =
        serde_json::from_str::<crate::admin_page_limit::AdminPageLimit>(constants_str::VALUE_0)
    else {
        std::panic::panic_any(constants_str::PANIC_E8FD3A29);
    };
    let Err(_above_maximum_error) =
        serde_json::from_str::<crate::admin_page_limit::AdminPageLimit>(constants_str::VALUE_101)
    else {
        std::panic::panic_any(constants_str::PANIC_36F08AD7);
    };
    let limit = <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(constants_str::VALUE_100),
    )
    .expect(constants_str::DIAGNOSTIC_A6AA5B42);
    let offset = <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
        serde::de::value::StrDeserializer::<serde::de::value::Error>::new(constants_str::VALUE_42),
    )
    .expect(constants_str::DIAGNOSTIC_799E47B0);
    assert_eq!(
        u16::from(limit),
        crate::admin_page_limit::AdminPageLimit::MAX
    );
    assert_eq!(u32::from(offset), 42u32);
}

#[test]
fn test_pagination_deserialization_enforces_signed_128_bit_boundaries() {
    assert!(
        [
            -1i128,
            0i128,
            1i128,
            100i128,
            101i128,
            i128::from(u16::MAX) + 1i128,
            i128::from(u32::MAX),
            i128::from(u32::MAX) + 1i128,
            i128::MAX
        ]
        .into_iter()
        .all(|number| {
            let limit =
                <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
                    serde::de::value::I128Deserializer::<serde::de::value::Error>::new(number),
                );
            let offset =
                <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
                    serde::de::value::I128Deserializer::<serde::de::value::Error>::new(number),
                );
            let limit_matches = if (1i128..=100i128).contains(&number) {
                limit.is_ok_and(|value| i128::from(u16::from(value)) == number)
            } else {
                limit.is_err()
            };
            let offset_matches = if (0i128..=i128::from(u32::MAX)).contains(&number) {
                offset.is_ok_and(|value| i128::from(u32::from(value)) == number)
            } else {
                offset.is_err()
            };
            limit_matches && offset_matches
        })
    );
}

#[test]
fn test_pagination_deserialization_enforces_unsigned_128_bit_boundaries() {
    assert!(
        [
            0u128,
            1u128,
            100u128,
            101u128,
            u128::from(u16::MAX) + 1u128,
            u128::from(u32::MAX),
            u128::from(u32::MAX) + 1u128,
            u128::MAX
        ]
        .into_iter()
        .all(|number| {
            let limit =
                <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
                    serde::de::value::U128Deserializer::<serde::de::value::Error>::new(number),
                );
            let offset =
                <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
                    serde::de::value::U128Deserializer::<serde::de::value::Error>::new(number),
                );
            let limit_matches = if (1u128..=100u128).contains(&number) {
                limit.is_ok_and(|value| u128::from(u16::from(value)) == number)
            } else {
                limit.is_err()
            };
            let offset_matches = if number <= u128::from(u32::MAX) {
                offset.is_ok_and(|value| u128::from(u32::from(value)) == number)
            } else {
                offset.is_err()
            };
            limit_matches && offset_matches
        })
    );
}

#[test]
fn test_pagination_wrong_json_types_preserve_expected_value_diagnostics() {
    assert!(
        [
            serde_json::json!(-1.0f64),
            serde_json::json!(0.0f64),
            serde_json::json!(1.0f64),
            serde_json::json!(1.5f64),
            serde_json::json!(100.0f64),
            serde_json::json!(true),
            serde_json::Value::Null,
            serde_json::json!([]),
            serde_json::json!({})
        ]
        .into_iter()
        .all(|wire| {
            let limit =
                serde_json::from_value::<crate::admin_page_limit::AdminPageLimit>(wire.clone());
            let offset = serde_json::from_value::<crate::admin_page_offset::AdminPageOffset>(wire);
            limit.is_err_and(|error| {
                let diagnostic = error.to_string();
                error.is_data()
                    && diagnostic
                        .contains(&crate::admin_page_limit::AdminPageLimit::MIN.to_string())
                    && diagnostic
                        .contains(&crate::admin_page_limit::AdminPageLimit::MAX.to_string())
            }) && offset.is_err_and(|error| {
                error.is_data()
                    && error
                        .to_string()
                        .contains(constants_str::ADMIN_PAGE_OFFSET_EXPECTING)
            })
        })
    );
}

#[test]
fn test_pagination_malformed_string_and_numeric_overflows_preserve_sources() {
    assert!(
        [
            constants_str::X.to_owned(),
            constants_str::EMPTY.to_owned(),
            constants_str::SPACE.to_owned(),
            (-1i64).to_string(),
            u64::MAX.to_string()
        ]
        .into_iter()
        .all(|text| {
            let expected_limit = text.parse::<u16>().err();
            let expected_offset = text.parse::<u32>().err();
            expected_limit
                .zip(expected_offset)
                .is_some_and(|(limit_source, offset_source)| {
                    serde_json::from_value::<crate::admin_page_limit::AdminPageLimit>(
                        serde_json::json!(&text),
                    )
                    .is_err_and(|error| error.to_string().contains(&limit_source.to_string()))
                        && serde_json::from_value::<crate::admin_page_offset::AdminPageOffset>(
                            serde_json::json!(text),
                        )
                        .is_err_and(|error| error.to_string().contains(&offset_source.to_string()))
                })
        })
    );
    let limit_source = u16::try_from(u64::MAX).err();
    let offset_source = u32::try_from(u64::MAX).err();
    assert!(
        limit_source
            .zip(offset_source)
            .is_some_and(|(expected_limit, expected_offset)| {
                serde_json::from_value::<crate::admin_page_limit::AdminPageLimit>(
                    serde_json::json!(u64::MAX),
                )
                .is_err_and(|error| error.to_string().contains(&expected_limit.to_string()))
                    && serde_json::from_value::<crate::admin_page_offset::AdminPageOffset>(
                        serde_json::json!(u64::MAX),
                    )
                    .is_err_and(|error| error.to_string().contains(&expected_offset.to_string()))
            })
    );
}

#[test]
fn test_pagination_integer_deserializers_preserve_exact_conversion_and_domain_errors() {
    assert!([i128::MIN, -1i128, i128::from(u64::MAX) + 1i128, i128::MAX]
        .into_iter().all(|integer| {
            u64::try_from(integer).err().is_some_and(|source| {
                let expected = source.to_string();
                <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
                    serde::de::value::I128Deserializer::<serde::de::value::Error>::new(integer),
                ).is_err_and(|error| error.to_string() == expected)
                    && <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
                        serde::de::value::I128Deserializer::<serde::de::value::Error>::new(integer),
                    ).is_err_and(|error| error.to_string() == expected)
            })
        }));
    assert!([u128::from(u64::MAX) + 1u128, u128::MAX]
        .into_iter().all(|integer| {
            u64::try_from(integer).err().is_some_and(|source| {
                let expected = source.to_string();
                <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
                    serde::de::value::U128Deserializer::<serde::de::value::Error>::new(integer),
                ).is_err_and(|error| error.to_string() == expected)
                    && <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
                        serde::de::value::U128Deserializer::<serde::de::value::Error>::new(integer),
                    ).is_err_and(|error| error.to_string() == expected)
            })
        }));
    assert!([u64::from(u32::MAX) + 1u64, u64::MAX]
        .into_iter().all(|integer| {
            u16::try_from(integer).err().zip(u32::try_from(integer).err())
                .is_some_and(|(limit_source, offset_source)| {
                    <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
                        serde::de::value::U64Deserializer::<serde::de::value::Error>::new(integer),
                    ).is_err_and(|error| error.to_string() == limit_source.to_string())
                        && <crate::admin_page_offset::AdminPageOffset as serde::Deserialize>::deserialize(
                            serde::de::value::U64Deserializer::<serde::de::value::Error>::new(integer),
                        ).is_err_and(|error| error.to_string() == offset_source.to_string())
                })
        }));
    let expected = crate::admin_page_limit_error::AdminPageLimitError::OutOfRange.to_string();
    assert!(
        [0u64, 101u64, u64::from(u16::MAX)]
            .into_iter()
            .all(|integer| {
                <crate::admin_page_limit::AdminPageLimit as serde::Deserialize>::deserialize(
                    serde::de::value::U64Deserializer::<serde::de::value::Error>::new(integer),
                )
                .is_err_and(|error| error.to_string() == expected)
            })
    );
}
