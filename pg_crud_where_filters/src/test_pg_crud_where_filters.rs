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
