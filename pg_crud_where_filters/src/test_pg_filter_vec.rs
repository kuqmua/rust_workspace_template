#[test]
fn test_filter_vector_binding_preserves_existing_arguments_and_empty_vectors() {
    assert!([false, true].into_iter().all(|has_existing_argument| {
        let initial = sqlx::query(constants_str::EMPTY);
        let query = if has_existing_argument {
            initial.bind(17i32)
        } else {
            initial
        };
        crate::pg_filter_vec::PgFilterVec::<i32, 0>::default()
            .query_bind(pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(query))
            .and_then(|sqlx_postgres_query| {
                crate::pg_filter_vec::PgFilterVec::from([1i32, 2i32, 3i32])
                    .query_bind(sqlx_postgres_query)
            })
            .is_ok_and(|bound| {
                let mut sqlx_query = bound.into_inner();
                sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                    arguments.is_some_and(|pg_arguments| {
                        sqlx::Arguments::len(&pg_arguments)
                            == if has_existing_argument {
                                4usize
                            } else {
                                3usize
                            }
                    })
                })
            })
    }));
}

#[test]
fn test_filter_vector_binding_retains_encoding_error_source() {
    #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
    struct FilterVectorFailureEncoder;

    impl sqlx::Type<sqlx::Postgres> for FilterVectorFailureEncoder {
        fn type_info() -> sqlx::postgres::PgTypeInfo {
            <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
        }
    }

    impl sqlx::Encode<'_, sqlx::Postgres> for FilterVectorFailureEncoder {
        fn encode_by_ref(
            &self,
            _pg_argument_buffer: &mut sqlx::postgres::PgArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
            Err(Box::new(std::io::Error::from(
                std::io::ErrorKind::InvalidData,
            )))
        }
    }

    assert!(
        crate::pg_filter_vec::PgFilterVec::from([FilterVectorFailureEncoder])
            .query_bind(
                pg_crud_common::sqlx_postgres_query::SqlxPostgresQuery::from(sqlx::query(
                    constants_str::EMPTY
                ),)
            )
            .is_err_and(|error| {
                std::error::Error::source(&error)
                    .and_then(std::error::Error::source)
                    .and_then(|source| source.downcast_ref::<std::io::Error>())
                    .is_some_and(|source| source.kind() == std::io::ErrorKind::InvalidData)
            })
    );
}

#[test]
fn test_filter_vector_length_diagnostics_report_observed_and_expected_counts() {
    assert!([0usize, 1usize, 3usize].into_iter().all(|count| {
        crate::pg_filter_vec::PgFilterVec::<i32, 2>::try_from(vec![1i32; count]).is_err_and(
            |error| {
                let diagnostic = error.to_string();
                [
                    format!("{}: {count}", stringify!(wrong_len)),
                    format!("{}: {}", stringify!(expected), 2usize),
                ]
                .into_iter()
                .all(|expected_line| diagnostic.lines().any(|line| line == expected_line))
            },
        )
    }));
}

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

#[test]
fn test_filter_vector_domain_default_preserves_exact_length_and_element_default() {
    #[derive(
        Clone,
        Default,
        proc_macro_optimal_memory_layout::OptimalMemoryLayout,
        proc_macro_newtype_from_inner::FromInner,
        proc_macro_newtype_get_inner::GetInner,
    )]
    #[borrow]
    struct FilterVectorCustomDefaultFixture(crate::encode_format::EncodeFormat);

    impl pg_crud_common::default_some_one_element::DefaultSomeOneElement
        for FilterVectorCustomDefaultFixture
    {
        fn default_some_one_element() -> Self {
            Self::from(crate::encode_format::EncodeFormat::Hex)
        }
    }

    assert_eq!(
        *FilterVectorCustomDefaultFixture::default().get(),
        crate::encode_format::EncodeFormat::Base64
    );
    let empty = <crate::pg_filter_vec::PgFilterVec<FilterVectorCustomDefaultFixture, 0usize> as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element();
    assert!(empty.as_slice().is_empty());
    let single = <crate::pg_filter_vec::PgFilterVec<FilterVectorCustomDefaultFixture, 1usize> as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element();
    assert_eq!(single.as_slice().len(), 1usize);
    assert!(
        single
            .as_slice()
            .iter()
            .all(|element| *element.get() == crate::encode_format::EncodeFormat::Hex)
    );
    let multiple = <crate::pg_filter_vec::PgFilterVec<FilterVectorCustomDefaultFixture, 3usize> as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element();
    assert_eq!(multiple.as_slice().len(), 3usize);
    assert!(
        multiple
            .as_slice()
            .iter()
            .all(|element| *element.get() == crate::encode_format::EncodeFormat::Hex)
    );
}
