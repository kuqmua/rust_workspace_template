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

#[test]
fn test_unique_vector_size_rejections_skip_duplicate_callback() {
    let maximum = crate::not_empty_unique_vec_max_len::NOT_EMPTY_UNIQUE_VEC_MAX_LEN;
    assert!([(Vec::new(), true), (vec![crate::duplicate_index::DuplicateIndex::from(0usize); maximum + 1usize], false)].into_iter().all(|(values, empty)| {
        let call_count = std::cell::Cell::new(0usize);
        let result = crate::try_new_unique_vec::try_new_unique_vec(values.into(), |_duplicate_candidates| {
            call_count.set(call_count.get() + 1usize);
            None
        });
        let expected_error = match result {
            Err(crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::IsEmpty { .. }) => empty,
            Err(crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::TooLong { .. }) => !empty,
            _ => false,
        };
        expected_error && call_count.get() == 0usize
    }));
}

#[test]
fn test_unique_vector_valid_sizes_invoke_duplicate_callback_once_and_preserve_result() {
    let first = crate::duplicate_index::DuplicateIndex::from(1usize);
    let second = crate::duplicate_index::DuplicateIndex::from(2usize);
    assert!([(vec![first, second], false), (vec![first, second, first], true)].into_iter().all(|(values, duplicated)| {
        let call_count = std::cell::Cell::new(0usize);
        let result = crate::try_new_unique_vec::try_new_unique_vec(values.into(), |duplicate_candidates| {
            call_count.set(call_count.get() + 1usize);
            crate::take_first_duplicate::take_first_duplicate(duplicate_candidates)
        });
        let correct_result = match result {
            Ok(validated_values) => !duplicated && validated_values == [first, second],
            Err(crate::not_empty_unique_vec_try_new_error::NotEmptyUniqueVecTryNewError::NotUnique { v, .. }) => duplicated && v == first,
            _ => false,
        };
        correct_result && call_count.get() == 1usize
    }));
}
