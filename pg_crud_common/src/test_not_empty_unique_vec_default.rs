#[test]
fn test_not_empty_unique_vec_default_contains_one_value() {
    let values = crate::not_empty_unique_vec::NotEmptyUniqueVec::<u8>::default();
    assert_eq!(values.as_slice(), &[0u8]);
}
