#[test]
fn test_unique_filter_deserialization_validates_nested_sequence() {
    let deserialize = |values: Vec<i32>| {
        let inner = serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
            values.into_iter(),
        );
        <crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec<i32> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new([inner].into_iter()),
        )
    };
    assert!(
        deserialize(vec![3i32, 1i32, 2i32])
            .is_ok_and(|values| values.as_slice() == [3i32, 1i32, 2i32])
    );
    assert!(matches!(
        deserialize(Vec::new()),
        Err(serde::de::value::Error { .. })
    ));
    assert!(matches!(
        deserialize(vec![1i32, 2i32, 1i32]),
        Err(serde::de::value::Error { .. })
    ));
}

#[test]
fn test_hash_unique_filter_validation_preserves_order_and_rejects_empty() {
    assert!(
        crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::try_from_by_hash(
            vec![3i32, 1i32, 2i32].into()
        )
        .is_ok_and(|values| values.as_slice() == [3i32, 1i32, 2i32])
    );
    assert!(matches!(crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::<i32>::try_from_by_hash(Vec::new().into()), Err(pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty { .. })));
}

#[test]
fn test_hash_unique_filter_capacity_is_inclusive() {
    let values = (0usize..10_000usize).collect::<Vec<_>>();
    assert!(
        crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::try_from_by_hash(
            values.clone().into()
        )
        .is_ok_and(|validated| validated.as_slice() == values)
    );
    assert!(matches!(crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec::try_from_by_hash((0usize..=10_000usize).collect::<Vec<_>>().into()), Err(pg_crud_common::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. })));
}

#[test]
fn test_unique_filter_deserialization_rejects_oversized_sequence() {
    let inner =
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(0usize..=10_000usize);
    let result = <crate::pg_type_not_empty_unique_vec::PgTypeNotEmptyUniqueVec<usize> as serde::Deserialize>::deserialize(
        serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new([inner].into_iter()),
    );
    assert!(matches!(result, Err(serde::de::value::Error { .. })));
}
