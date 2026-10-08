#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype_from_inner::FromInner,
)]
struct NonClone(u8);

#[test]
fn test_pg_filter_vec_default_respects_fixed_length() {
    let default = crate::pg_filter_vec::PgFilterVec::<u8, 2>::default();
    assert_eq!(default.as_slice().len(), 2);
}

#[test]
fn test_pg_type_not_empty_unique_vec_default_contains_element() {
    let default = crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<u8>::default();
    assert_eq!(default.as_slice().len(), 1);
}

#[test]
fn test_between_accepts_equal_inclusive_bounds() {
    assert!(
        crate::between::Between::<i32>::try_new(5i32, 5i32)
            .ok()
            .is_some()
    );
    assert!(matches!(
        crate::between::Between::<i32>::try_new(6i32, 5i32),
        Err(crate::between_try_new_error::BetweenTryNewError::StartNotLessThanOrEqualToEnd { .. })
    ));
    assert!(matches!(
        crate::between::Between::<f32>::try_new(f32::NAN, 5.0),
        Err(crate::between_try_new_error::BetweenTryNewError::StartNotLessThanOrEqualToEnd { .. })
    ));
}

#[test]
fn test_pg_type_not_empty_unique_vec_try_from_ok() {
    let rslt = crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<i32>::try_from(vec![
        1i32, 2i32, 3i32,
    ]);
    if let Err(error) = rslt {
        std::panic::panic_any(constants_str::PANIC_5A6AFCFA.replacen(
            constants_str::PANIC_PLACEHOLDER_04CEF635,
            format!("{error:?}").as_str(),
            1usize,
        ));
    }
}

#[test]
fn test_pg_type_not_empty_unique_vec_try_from_empty() {
    let rslt =
        crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<i32>::try_from(Vec::new());
    assert!(matches!(
        rslt,
        Err(pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty { .. })
    ));
}

#[test]
fn test_pg_type_not_empty_unique_vec_try_from_not_unique() {
    let rslt = crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<i32>::try_from(vec![
        1i32, 2i32, 1i32,
    ]);
    assert!(matches!(
        rslt,
        Err(pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique { v: 1i32, .. })
    ));
}

#[test]
fn test_pg_type_not_empty_unique_vec_try_from_too_long() {
    let rslt = crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<usize>::try_from(
        (constants_usize::ZERO..=10_000usize).collect::<Vec<_>>(),
    );
    assert!(matches!(
        rslt,
        Err(pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. })
    ));
}

#[test]
fn test_pg_type_not_empty_unique_vec_try_from_by_hash_not_unique() {
    let rslt =
        crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<i32>::try_from_by_hash(
            vec![1i32, 2i32, 1i32].into(),
        );
    assert!(matches!(
        rslt,
        Err(pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique { v: 1i32, .. })
    ));
}

#[test]
fn test_pg_type_not_empty_unique_vec_try_from_supports_non_clone_values() {
    let rslt =
        crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<NonClone>::try_from(vec![
            NonClone(1),
            NonClone(2),
            NonClone(1),
        ]);
    assert!(matches!(
        rslt,
        Err(
            pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique {
                v: NonClone(1),
                ..
            }
        )
    ));
}

#[test]
fn test_encode_format_display_is_stable() {
    assert_eq!(
        crate::encode_format::EncodeFormat::Base64.to_string(),
        constants_str::VALUE_371A286D
    );
    assert_eq!(
        crate::encode_format::EncodeFormat::Escape.to_string(),
        constants_str::VALUE_B3140286
    );
    assert_eq!(
        crate::encode_format::EncodeFormat::Hex.to_string(),
        constants_str::VALUE_128DF13C
    );
}

#[test]
fn test_encode_format_defaults_and_deserialization_preserve_wire_names() {
    assert_eq!(
        crate::encode_format::EncodeFormat::default(),
        crate::encode_format::EncodeFormat::Base64
    );
    assert_eq!(
        <crate::encode_format::EncodeFormat as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element(),
        crate::encode_format::EncodeFormat::Base64
    );
    [
        (
            stringify!(Base64),
            crate::encode_format::EncodeFormat::Base64,
        ),
        (
            stringify!(Escape),
            crate::encode_format::EncodeFormat::Escape,
        ),
        (stringify!(Hex), crate::encode_format::EncodeFormat::Hex),
    ]
    .into_iter()
    .fold((), |(), (wire_name, encode_format)| {
        assert_eq!(
            <crate::encode_format::EncodeFormat as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(wire_name)
            ),
            Ok(encode_format)
        );
    });
    [
        constants_str::EMPTY,
        constants_str::BAD,
        constants_str::VALUE_371A286D,
        constants_str::VALUE_B3140286,
        constants_str::VALUE_128DF13C,
    ]
    .into_iter()
    .fold((), |(), wire_name| {
        assert!(matches!(
            <crate::encode_format::EncodeFormat as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(wire_name)
            ),
            Err(serde::de::value::Error { .. })
        ));
    });
}

#[test]
fn test_filter_enum_schemas_advertise_exact_wire_names() {
    [
        (
            schemars::schema_for!(crate::encode_format::EncodeFormat),
            <crate::encode_format::EncodeFormat as utoipa::PartialSchema>::schema(),
            vec![Some(stringify!(Base64)), Some(stringify!(Escape)), Some(stringify!(Hex))],
        ),
        (
            schemars::schema_for!(crate::regex_case::RegexCase),
            <crate::regex_case::RegexCase as utoipa::PartialSchema>::schema(),
            vec![Some(stringify!(Insensitive)), Some(stringify!(Sensitive))],
        ),
    ].into_iter().fold((), |(), (json_schema, openapi_schema, expected)| {
        assert!(json_schema.as_value().get(stringify!(enum)).and_then(|value| value.as_array())
            .is_some_and(|variants| variants.iter().map(|value| value.as_str()).eq(expected.iter().copied())));
        assert!(matches!(openapi_schema,
            utoipa::openapi::RefOr::T(utoipa::openapi::schema::Schema::Object(object))
            if object.enum_values.as_ref().is_some_and(|variants| variants.iter().map(|value| value.as_str()).eq(expected.iter().copied()))
        ));
    });
}

#[test]
fn test_regex_case_default_and_deserialization_preserve_wire_names() {
    assert_eq!(
        <crate::regex_case::RegexCase as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element(),
        crate::regex_case::RegexCase::Sensitive
    );
    [
        (
            stringify!(Insensitive),
            crate::regex_case::RegexCase::Insensitive,
            constants_str::ASTERISK_ALT,
        ),
        (
            stringify!(Sensitive),
            crate::regex_case::RegexCase::Sensitive,
            constants_str::TEXT_ALT_15,
        ),
    ]
    .into_iter()
    .fold((), |(), (wire_name, regex_case, syntax)| {
        assert_eq!(
            <crate::regex_case::RegexCase as serde::Deserialize>::deserialize(
                serde::de::value::StrDeserializer::<serde::de::value::Error>::new(wire_name)
            ),
            Ok(regex_case)
        );
        assert_eq!(regex_case.postgreql_syntax().as_ref(), syntax);
        assert!(matches!(
            <crate::regex_case::RegexCase as serde::Deserialize>::deserialize(
                serde::de::value::StringDeserializer::<serde::de::value::Error>::new(
                    wire_name.to_lowercase()
                )
            ),
            Err(serde::de::value::Error { .. })
        ));
    });
    [constants_str::EMPTY, constants_str::BAD]
        .into_iter()
        .fold((), |(), wire_name| {
            assert!(matches!(
                <crate::regex_case::RegexCase as serde::Deserialize>::deserialize(
                    serde::de::value::StrDeserializer::<serde::de::value::Error>::new(wire_name)
                ),
                Err(serde::de::value::Error { .. })
            ));
        });
}

#[test]
fn test_regex_regex_eq_compares_pattern_content() {
    let left = crate::regex_regex::RegexRegex::try_from(String::from(constants_str::D_PLUS))
        .expect(constants_str::DIAGNOSTIC_8342AD27);
    let right = crate::regex_regex::RegexRegex::try_from(String::from(constants_str::D_PLUS))
        .expect(constants_str::DIAGNOSTIC_4D0FA8E3);
    let other = crate::regex_regex::RegexRegex::try_from(String::from(constants_str::A_Z_PLUS))
        .expect(constants_str::DIAGNOSTIC_ABCC9A72);
    assert_eq!(left, right);
    assert_ne!(left, other);
    assert_eq!(
        other,
        crate::regex_regex::RegexRegex::from(crate::default_regex_pattern::DefaultRegexPattern),
    );
}

#[test]
fn test_regex_regex_accepts_postgres_word_start_escape() {
    let pattern = crate::regex_regex::RegexRegex::try_from(String::from(
        constants_str::PG_CRUD_REGEX_POSTGRES_WORD_START,
    ));
    assert!(pattern.ok().is_some());
}

#[test]
fn test_regex_regex_preserves_oversized_pattern_length() {
    let length = constants_usize::VALUE_1_048_576 + constants_usize::ONE;
    let pattern = std::iter::repeat_n('x', length).collect::<String>();
    assert!(matches!(
        crate::regex_regex::RegexRegex::try_from(pattern),
        Err(crate::regex_regex_try_from_string_error::RegexRegexTryFromStringError::TooLong {
            source: bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                actual_length,
                maximum_length,
            },
        }) if actual_length == bounded_types::bounded_len::BoundedLen::from(length)
            && maximum_length
                == bounded_types::bounded_len::BoundedLen::from(constants_usize::VALUE_1_048_576)
    ));
}

#[test]
fn test_regex_pattern_deserialization_validates_byte_limits_and_preserves_opaque_text() {
    assert!(
        [
            (String::new(), true),
            ('['.to_string(), true),
            (constants_str::U_1F496.repeat(262_144usize), true),
            (
                format!(
                    "{}{}",
                    constants_str::U_1F496.repeat(262_144usize),
                    constants_str::X
                ),
                false
            ),
        ]
        .into_iter()
        .all(|(text, accepted)| {
            let direct = crate::regex_regex::RegexRegex::try_from(text.clone());
            let decoded = <crate::regex_regex::RegexRegex as serde::Deserialize>::deserialize(
                serde::de::value::StringDeserializer::<serde::de::value::Error>::new(text.clone()),
            );
            if accepted {
                direct.is_ok_and(|pattern| pattern.to_string() == text)
                    && decoded.is_ok_and(|pattern| pattern.to_string() == text)
            } else {
                direct.is_err() && decoded.is_err()
            }
        })
    );
}
