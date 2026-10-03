#[test]
fn test_filter_vec_conversion_requires_exact_length() {
    assert!(
        crate::pg_filter_vec::PgFilterVec::<i32, 2>::try_from(vec![1i32, 2i32])
            .is_ok_and(|value| value.as_slice() == [1i32, 2i32])
    );
    assert!(matches!(
        crate::pg_filter_vec::PgFilterVec::<i32, 2>::try_from(vec![1i32]),
        Err(crate::bounded_vec_try_new_error::BoundedVecTryNewError::LenIsNotCorrect { wrong_len, expected, .. })
            if wrong_len == crate::pg_filter_vec_len::PgFilterVecLen::from(1usize)
                && expected == crate::pg_filter_vec_len::PgFilterVecLen::from(2usize)
    ));
    assert!(matches!(
        crate::pg_filter_vec::PgFilterVec::<i32, 2>::try_from(vec![1i32, 2i32, 3i32]),
        Err(crate::bounded_vec_try_new_error::BoundedVecTryNewError::LenIsNotCorrect { wrong_len, expected, .. })
            if wrong_len == crate::pg_filter_vec_len::PgFilterVecLen::from(3usize)
                && expected == crate::pg_filter_vec_len::PgFilterVecLen::from(2usize)
    ));
}

#[test]
fn test_filter_vec_query_parts_advance_placeholder_counter() {
    let values = crate::pg_filter_vec::PgFilterVec::from([1i32, 2i32, 3i32]);
    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(4u64);
    let column = constants_str::TEST_DB_COLUMN_ID;
    let normal = values.pg_type_query_part(
        &mut increment,
        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
        pg_crud_common::add_operator::AddOperator::from(false),
    );
    assert!(normal.is_ok_and(|fragment| fragment.as_ref() == format!("[${}][${}][${}]", 5u64, 6u64, 7u64)));
    assert_eq!(increment.get(), 7u64);
    let minus_one = values.pg_type_query_part_minus_one(
        &mut increment,
        pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
        pg_crud_common::add_operator::AddOperator::from(true),
    );
    assert!(minus_one.is_ok_and(|fragment| fragment.as_ref() == format!("[${}][${}]", 8u64, 9u64)));
    assert_eq!(increment.get(), 9u64);
}

#[test]
fn test_empty_filter_vec_query_does_not_increment() {
    let values = crate::pg_filter_vec::PgFilterVec::<i32, 0>::default();
    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(u64::MAX);
    let column = constants_str::TEST_DB_COLUMN_ID;
    assert!(
        values
            .pg_type_query_part(
                &mut increment,
                pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                false.into()
            )
            .is_ok_and(|fragment| fragment.as_ref().is_empty())
    );
    assert!(
        values
            .pg_type_query_part_minus_one(
                &mut increment,
                pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                false.into()
            )
            .is_ok_and(|fragment| fragment.as_ref().is_empty())
    );
    assert_eq!(increment.get(), u64::MAX);
}

#[test]
fn test_filter_vec_query_overflow_preserves_last_successful_increment() {
    let values = crate::pg_filter_vec::PgFilterVec::from([1i32, 2i32]);
    let mut increment =
        pg_crud_common::query_part_increment::QueryPartIncrement::from(u64::MAX - 1u64);
    let column = constants_str::TEST_DB_COLUMN_ID;
    assert!(matches!(
        values.pg_type_query_part(
            &mut increment,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            false.into()
        ),
        Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })
    ));
    assert_eq!(increment.get(), u64::MAX);
}

#[test]
fn test_filter_vec_deserialization_requires_exact_length() {
    let deserialize = |values: Vec<i32>| {
        <crate::pg_filter_vec::PgFilterVec<i32, 2> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                values.into_iter(),
            ),
        )
    };
    assert!(deserialize(vec![1i32, 2i32]).is_ok_and(|values| values.as_slice() == [1i32, 2i32]));
    assert!(matches!(
        deserialize(Vec::new()),
        Err(serde::de::value::Error { .. })
    ));
    assert!(matches!(
        deserialize(vec![1i32]),
        Err(serde::de::value::Error { .. })
    ));
    assert!(matches!(
        deserialize(vec![1i32, 2i32, 3i32]),
        Err(serde::de::value::Error { .. })
    ));
}

#[test]
fn test_zero_length_filter_vec_deserialization_accepts_only_empty_sequence() {
    let deserialize = |values: Vec<i32>| {
        <crate::pg_filter_vec::PgFilterVec<i32, 0> as serde::Deserialize>::deserialize(
            serde::de::value::SeqDeserializer::<_, serde::de::value::Error>::new(
                values.into_iter(),
            ),
        )
    };
    assert!(deserialize(Vec::new()).is_ok_and(|values| values.as_slice().is_empty()));
    assert!(matches!(
        deserialize(vec![1i32]),
        Err(serde::de::value::Error { .. })
    ));
}

#[test]
fn test_single_filter_minus_one_does_not_increment() {
    let values = crate::pg_filter_vec::PgFilterVec::from([1i32]);
    let mut increment = pg_crud_common::query_part_increment::QueryPartIncrement::from(u64::MAX);
    let column = constants_str::TEST_DB_COLUMN_ID;
    assert!(
        values
            .pg_type_query_part_minus_one(
                &mut increment,
                pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
                true.into()
            )
            .is_ok_and(|fragment| fragment.as_ref().is_empty())
    );
    assert_eq!(increment.get(), u64::MAX);
    assert!(matches!(
        values.pg_type_query_part(
            &mut increment,
            pg_crud_common::sql_column_ref::SqlColumnRef::from(&column),
            true.into()
        ),
        Err(pg_crud_common::query_part_error::QueryPartError::CheckedAdd { .. })
    ));
    assert_eq!(increment.get(), u64::MAX);
}
