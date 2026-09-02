#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::FromInner,
)]
struct NonClone(u8);

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
}
