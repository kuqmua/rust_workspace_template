#[cfg(feature = "test-utils")]
#[must_use]
pub fn string_test_cases_vec() -> [String; 12] {
    [
        String::new(),
        constants_str::A_ALT.to_owned(),
        constants_str::HELLO_WORLD.to_owned(),
        constants_str::THREE_SPACES.to_owned(),
        constants_str::NEWLINE_CARRIAGE_RETURN_TAB.to_owned(),
        constants_str::VALUE_1234567890.to_owned(),
        constants_str::U_1F600.to_owned(),
        constants_str::U_3053_U_3093_U_306B_U_3061_U_306F.to_owned(),
        constants_str::U_1F30D_U_1F680_U_2728_RUST_U_1F496_U_1F980.to_owned(),
        constants_str::A_ALT.repeat(1024),
        constants_str::LINE1_NEWLINE_LINE2_NEWLINE_LINE3.to_owned(),
        constants_str::U_1F496.to_owned(),
    ]
}

#[cfg(feature = "test-utils")]
#[cfg(test)]
mod tests {
    #[test]
    fn test_boolean_fixtures_preserve_both_values_and_order() {
        assert_eq!(
            crate::bool_test_cases_vec::bool_test_cases_vec(),
            [true, false]
        );
    }

    #[test]
    fn test_integer_fixtures_preserve_extremes_zero_and_unsigned_duplicates() {
        assert_eq!(
            crate::i8_test_cases_vec::i8_test_cases_vec(),
            [i8::MIN, 0, i8::MAX]
        );
        assert_eq!(
            crate::i16_test_cases_vec::i16_test_cases_vec(),
            [i16::MIN, 0, i16::MAX]
        );
        assert_eq!(
            crate::i32_test_cases_vec::i32_test_cases_vec(),
            [i32::MIN, 0i32, i32::MAX]
        );
        assert_eq!(
            crate::i64_test_cases_vec::i64_test_cases_vec(),
            [i64::MIN, 0, i64::MAX]
        );
        assert_eq!(
            crate::u8_test_cases_vec::u8_test_cases_vec(),
            [u8::MIN, 0, u8::MAX]
        );
        assert_eq!(
            crate::u16_test_cases_vec::u16_test_cases_vec(),
            [u16::MIN, 0, u16::MAX]
        );
        assert_eq!(
            crate::u32_test_cases_vec::u32_test_cases_vec(),
            [u32::MIN, 0, u32::MAX]
        );
        assert_eq!(
            crate::u64_test_cases_vec::u64_test_cases_vec(),
            [u64::MIN, 0, u64::MAX]
        );
    }

    #[test]
    fn test_text_fixtures_preserve_distinct_empty_whitespace_unicode_and_long_cases() {
        let cases = crate::string_test_cases_vec::string_test_cases_vec();
        assert_eq!(
            cases
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            cases.len()
        );
        assert!(cases.iter().any(String::is_empty));
        assert!(
            cases
                .iter()
                .any(|value| !value.is_empty()
                    && value.bytes().all(|byte| byte.is_ascii_whitespace()))
        );
        assert!(
            cases
                .iter()
                .any(|value| value.chars().any(char::is_control))
        );
        assert!(cases.iter().any(|value| !value.is_ascii()));
        assert!(
            cases
                .iter()
                .any(|value| value.len() == 1024usize && value.is_ascii())
        );
        assert!(cases.iter().any(|value| value.lines().count() >= 3usize));
        assert!(cases.into_iter().all(|value| {
            crate::query_part_fragment::QueryPartFragment::try_from(value.clone())
                .is_ok_and(|validated| validated.as_ref() == value)
        }));
    }

    #[test]
    fn test_float_fixtures_preserve_finite_extremes_signed_zeros_and_json_bits() {
        assert!(
            [
                (
                    crate::f32_test_cases_vec::f32_test_cases_vec()
                        .into_iter()
                        .map(f64::from)
                        .collect::<Vec<_>>(),
                    f64::from(f32::MAX),
                    f64::from(f32::MIN_POSITIVE),
                ),
                (
                    crate::f64_test_cases_vec::f64_test_cases_vec().to_vec(),
                    f64::MAX,
                    f64::MIN_POSITIVE
                ),
            ]
            .into_iter()
            .all(|(values, maximum, minimum_positive)| {
                values.iter().all(|value| value.is_finite())
                    && values
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<std::collections::BTreeSet<_>>()
                        .len()
                        == values.len()
                    && [maximum, -maximum, minimum_positive, 0.0f64, -0.0f64]
                        .into_iter()
                        .all(|required| {
                            values
                                .iter()
                                .any(|value| value.to_bits() == required.to_bits())
                        })
                    && values.into_iter().all(|value| {
                        serde_json::to_value(value).is_ok_and(|json| {
                            serde_json::from_value::<f64>(json)
                                .is_ok_and(|decoded| decoded.to_bits() == value.to_bits())
                        })
                    })
            })
        );
    }

    #[test]
    fn test_uuid_fixtures_preserve_fixed_non_nil_version_four_values() {
        let first = crate::uuid_uuid_test_cases_vec::uuid_uuid_test_cases_vec()
            .into_iter()
            .collect::<Vec<_>>();
        let second = crate::uuid_uuid_test_cases_vec::uuid_uuid_test_cases_vec()
            .into_iter()
            .collect::<Vec<_>>();
        assert_eq!(first, second);
        assert!(!first.is_empty());
        assert!(first.into_iter().all(|value| {
            !value.is_nil()
                && value.get_version() == Some(uuid::Version::Random)
                && value.get_variant() == uuid::Variant::RFC4122
        }));
    }
}
