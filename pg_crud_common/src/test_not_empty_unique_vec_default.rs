#[test]
fn test_not_empty_unique_vec_default_contains_one_value() {
    let values = crate::not_empty_unique_vec::NotEmptyUniqueVec::<u8>::default();
    assert_eq!(values.as_slice(), &[0u8]);
}

#[test]
fn test_unique_vector_hash_construction_preserves_order_at_length_boundaries() {
    let maximum = crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN;
    assert!([1usize, 3usize, maximum].into_iter().all(|length| {
        let expected = (0usize..length).rev().collect::<Vec<_>>();
        crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new_by_hash(
            crate::duplicate_candidates::DuplicateCandidates::from(expected.clone()),
        )
        .is_ok_and(|validated| validated.as_slice() == expected)
    }));
}

#[test]
fn test_unique_vector_hash_construction_checks_length_before_duplicates() {
    let maximum = crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN;
    assert!(matches!(
        crate::not_empty_unique_vec::NotEmptyUniqueVec::<usize>::try_new_by_hash(
            crate::duplicate_candidates::DuplicateCandidates::from(Vec::new()),
        ),
        Err(
            crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty { .. }
        )
    ));
    assert!([
        (0usize..=maximum).collect::<Vec<_>>(),
        vec![7usize; maximum + 1usize],
    ].into_iter().all(|values| {
        matches!(crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new_by_hash(
            crate::duplicate_candidates::DuplicateCandidates::from(values.clone()),
        ), Err(crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. }))
            && matches!(crate::not_empty_unique_vec::NotEmptyUniqueVec::try_from(
                crate::duplicate_candidates::DuplicateCandidates::from(values),
            ), Err(crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. }))
    }));
}

#[test]
fn test_unique_vector_json_round_trip_preserves_exact_limit_and_rejects_invalid_values() {
    let maximum = crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN;
    let round_trips_preserve_values = [1usize, 3usize, maximum].into_iter().all(|length| {
        let expected = (0usize..length).rev().collect::<Vec<_>>();
        crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new_by_hash(
            crate::duplicate_candidates::DuplicateCandidates::from(expected.clone()),
        )
        .is_ok_and(|validated| {
            serde_json::to_value(&validated).is_ok_and(|json| {
                json == serde_json::Value::Array(
                        expected
                            .iter()
                            .copied()
                            .map(serde_json::Value::from)
                            .collect(),
                    ) && serde_json::from_value::<
                        crate::not_empty_unique_vec::NotEmptyUniqueVec<usize>,
                    >(json)
                    .is_ok_and(|decoded| decoded.as_slice() == expected)
            })
        })
    });
    assert!(round_trips_preserve_values);
    assert!([
        Vec::new(),
        vec![7usize, 7usize],
        (0usize..=maximum).collect::<Vec<_>>(),
    ].into_iter().all(|values| {
        let json = serde_json::Value::Array(values.into_iter().map(serde_json::Value::from).collect());
        matches!(serde_json::from_value::<crate::not_empty_unique_vec::NotEmptyUniqueVec<usize>>(json), Err(error) if error.is_data())
    }));
}

#[test]
fn test_unique_vector_element_conversion_preserves_length_and_order() {
    assert!([vec![7u8], vec![7u8, 3u8, 9u8]].into_iter().all(|values| {
        let expected = values.iter().copied().map(u16::from).collect::<Vec<_>>();
        crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
            crate::duplicate_candidates::DuplicateCandidates::from(values),
        )
        .is_ok_and(|validated| {
            crate::not_empty_unique_vec::NotEmptyUniqueVec::from_t1_impl_from_t2::<u16>(validated)
                .as_slice()
                == expected
        })
    }));
}
