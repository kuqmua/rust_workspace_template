#[derive(Debug, Clone, PartialEq, Eq, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement);

#[derive(Debug, Clone, PartialEq, Eq, proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
enum TestFallibleWhereValue {
    Accepted(crate::query_part_increment::QueryPartIncrement),
    Rejected(crate::query_part_increment::QueryPartIncrement),
}

impl<'query_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt>
    for TestFallibleWhereValue
{
    fn query_bind(
        self,
        mut sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        match self {
            Self::Accepted(query_part_increment) => {
                let value = i64::try_from(query_part_increment.get())
                    .map_err(crate::make_query_bind_error::make_query_bind_error)?;
                sqlx_postgres_query.as_mut().try_bind(value).map_err(
                    crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError::from,
                )?;
                Ok(sqlx_postgres_query)
            }
            Self::Rejected(_) => Err(crate::make_query_bind_error::make_query_bind_error(
                std::io::Error::from(std::io::ErrorKind::InvalidData),
            )),
        }
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        let _: crate::sql_column_ref::SqlColumnRef<'_> = sql_column_ref;
        let _: crate::query_part_increment::QueryPartIncrement = crate::increment_checked_add_one_returning_increment::increment_checked_add_one_returning_increment(increment)?;
        match self {
            Self::Accepted(_) => Ok(crate::query_part_fragment::QueryPartFragment::try_from(
                format!(
                    "{}{}",
                    if bool::from(add_operator) {
                        constants_str::AND_ALT
                    } else {
                        constants_str::EMPTY
                    },
                    constants_str::X
                ),
            )?),
            Self::Rejected(_) => Err(crate::query_part_error::QueryPartError::CheckedAdd {
                location: proc_macro_location_bang::location!(),
            }),
        }
    }
}

#[test]
fn test_unique_vector_query_forwards_first_operator_and_joins_later_elements() {
    assert!([false, true].into_iter().all(|add_operator| {
        crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
            crate::duplicate_candidates::DuplicateCandidates::from(vec![
                TestFallibleWhereValue::Accepted(
                    crate::query_part_increment::QueryPartIncrement::from(1u64),
                ),
                TestFallibleWhereValue::Accepted(
                    crate::query_part_increment::QueryPartIncrement::from(2u64),
                ),
            ]),
        )
        .is_ok_and(|values| {
            let mut increment = crate::query_part_increment::QueryPartIncrement::from(7u64);
            crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &values,
                &mut increment,
                crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                crate::add_operator::AddOperator::from(add_operator),
            )
            .is_ok_and(|fragment| {
                fragment.as_ref()
                    == format!(
                        "{}{}{}{}",
                        if add_operator {
                            constants_str::AND_ALT
                        } else {
                            constants_str::EMPTY
                        },
                        constants_str::X,
                        constants_str::AND_ALT,
                        constants_str::X
                    )
                    && increment.get() == 9u64
            })
        })
    }));
}

impl crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement for TestFallibleWhereValue {
    fn all_variants_default_some_one_element() -> crate::all_enum_variants::AllEnumVariants<Self> {
        vec![Self::Accepted(crate::query_part_increment::QueryPartIncrement::from(1u64))].into()
    }
}

#[test]
fn test_unique_vector_child_errors_preserve_progress_and_stop_later_elements() {
    assert!([false, true].into_iter().all(|has_prior_success| {
        let mut candidates = Vec::new();
        if has_prior_success {
            candidates.push(TestFallibleWhereValue::Accepted(
                crate::query_part_increment::QueryPartIncrement::from(1u64),
            ));
        }
        candidates.extend([
            TestFallibleWhereValue::Rejected(
                crate::query_part_increment::QueryPartIncrement::from(2u64),
            ),
            TestFallibleWhereValue::Accepted(
                crate::query_part_increment::QueryPartIncrement::from(3u64),
            ),
        ]);
        crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
            crate::duplicate_candidates::DuplicateCandidates::from(candidates),
        )
        .is_ok_and(|values| {
            let mut increment = crate::query_part_increment::QueryPartIncrement::from(7u64);
            matches!(
                crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                    &values,
                    &mut increment,
                    crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                    crate::add_operator::AddOperator::from(false),
                ),
                Err(crate::query_part_error::QueryPartError::CheckedAdd { .. })
            ) && increment.get() == if has_prior_success { 9u64 } else { 8u64 }
        })
    }));
}

impl<'query_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt>
    for TestOversizedWhereValue
{
    fn query_bind(
        self,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        Ok(sqlx_postgres_query)
    }

    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        let _: (
            &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
            crate::sql_column_ref::SqlColumnRef<'_>,
            crate::add_operator::AddOperator,
        ) = (increment, sql_column_ref, add_operator);
        Ok(crate::query_part_fragment::QueryPartFragment::try_from(
            constants_str::X.repeat(600_000usize),
        )?)
    }
}

impl crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement
    for TestOversizedWhereValue
{
    fn all_variants_default_some_one_element() -> crate::all_enum_variants::AllEnumVariants<Self> {
        vec![Self(crate::query_part_increment::QueryPartIncrement::from(1u64))].into()
    }
}

impl crate::all_enum_variants_array_default_some_one_element_max_page_size::AllEnumVariantsArrayDefaultSomeOneElementMaxPageSize
    for TestOversizedWhereValue
{
    fn all_variants_default_some_one_element_max_page_size() -> crate::all_enum_variants::AllEnumVariants<Self> {
        vec![
            Self(crate::query_part_increment::QueryPartIncrement::from(2u64)),
            Self(crate::query_part_increment::QueryPartIncrement::from(1u64)),
        ].into()
    }
}

#[test]
fn test_unique_vector_maximum_page_default_preserves_declared_variant_order() {
    let values = <crate::not_empty_unique_vec::NotEmptyUniqueVec<TestOversizedWhereValue> as crate::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize>::default_some_one_element_max_page_size();
    assert_eq!(
        values.as_slice(),
        &[
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(2u64)),
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(1u64)),
        ]
    );
}

#[test]
fn test_query_fragment_error_conversion_preserves_bounded_diagnostic_text() {
    let maximum = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
    let error = crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
        len: maximum + 1,
        max: maximum,
    };
    let expected = error.to_string();
    let fragment = crate::query_part_fragment::QueryPartFragment::from(error);
    assert_eq!(fragment.as_ref(), expected);
    assert!(fragment.as_ref().len() <= maximum);
    assert!(
        crate::query_part_fragment::QueryPartFragment::try_from(fragment.into_inner())
            .is_ok_and(|validated| validated.as_ref() == expected)
    );
}

#[test]
fn test_read_bind_index_append_preserves_exact_fit_and_partial_progress_on_overflow() {
    let maximum = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
    let decimal = u32::MAX.to_string();
    assert!([0usize, 2usize, decimal.len()].into_iter().all(|available| {
        let prefix_length = maximum - available;
        crate::query_part_fragment::QueryPartFragment::try_from(constants_str::X.repeat(prefix_length))
            .is_ok_and(|mut fragment| {
                let result = fragment.append_read_bind_index(
                    crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32::from(std::num::NonZeroU32::MAX),
                );
                let expected_result = if available == decimal.len() {
                    Ok(())
                } else {
                    Err(crate::read_query_plan_error::ReadQueryPlanError::TooManyFragments)
                };
                result == expected_result
                    && fragment.as_ref().len() == maximum
                    && fragment.as_ref().get(..prefix_length).is_some_and(|prefix| {
                        prefix.bytes().all(|byte| constants_str::X.as_bytes().first() == Some(&byte))
                    })
                    && decimal.get(..available).is_some_and(|suffix| fragment.as_ref().get(prefix_length..) == Some(suffix))
            })
    }));
}

#[test]
fn test_read_bind_index_decimal_digits_preserve_full_unsigned_range() {
    assert!([1u32, 9u32, 10u32, 1_012_345_678u32, u32::MAX]
        .into_iter()
        .all(|value| {
            let non_zero_result = std::num::NonZeroU32::new(value);
            let fragment_result = crate::query_part_fragment::QueryPartFragment::try_from(String::new());
            match (non_zero_result, fragment_result) {
                (Some(non_zero), Ok(mut fragment)) => {
                    fragment
                        .append_read_bind_index(
                            crate::read_query_bind_index_non_zero_u32::ReadQueryBindIndexNonZeroU32::from(non_zero),
                        )
                        .is_ok_and(|()| fragment.as_ref() == value.to_string())
                }
                _ => false,
            }
        }));
}

#[test]
fn test_fragment_append_accepts_exact_limit_and_preserves_full_buffer_on_rejection() {
    let maximum = crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN;
    let fragment = crate::query_part_fragment::QueryPartFragment::try_from(
        constants_str::X.repeat(maximum - 1),
    );
    assert!(fragment.is_ok_and(|mut value| {
        std::fmt::Write::write_str(&mut value, constants_str::X) == Ok(())
            && value.as_ref().len() == maximum
            && std::fmt::Write::write_str(&mut value, constants_str::EMPTY) == Ok(())
            && std::fmt::Write::write_str(&mut value, constants_str::X) == Err(std::fmt::Error)
            && value.as_ref() == constants_str::X.repeat(maximum)
    }));
    assert!(matches!(
        crate::query_part_fragment::QueryPartFragment::try_from(
            constants_str::X.repeat(maximum + 1),
        )
        , Err(crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong { len, max }) if len == maximum + 1 && max == maximum
    ));
}

#[test]
fn test_pg_type_where_rejects_combined_fragment_overflow() {
    let filter_result = crate::pg_type_where::PgTypeWhere::try_new(
        crate::operator::Operator::Or,
        crate::duplicate_candidates::DuplicateCandidates::from(vec![
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(1u64)),
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(2u64)),
        ]),
    );
    assert!(filter_result.is_ok());
    if let Ok(filter) = filter_result {
        let mut increment = crate::query_part_increment::QueryPartIncrement::from(0u64);
        assert!(matches!(
            crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &filter,
                &mut increment,
                crate::sql_column_ref::SqlColumnRef::from(&constants_str::X),
                crate::add_operator::AddOperator::from(false),
            ),
            Err(crate::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
        ));
    }
}

#[test]
fn test_pg_type_where_one_element_default_preserves_operator_and_variant_values() {
    let filter = <crate::pg_type_where::PgTypeWhere<TestOversizedWhereValue> as crate::default_some_one_element::DefaultSomeOneElement>::default_some_one_element();
    assert_eq!(filter.operator(), &crate::operator::Operator::Or);
    assert_eq!(
        filter.values().as_slice(),
        &[TestOversizedWhereValue(
            crate::query_part_increment::QueryPartIncrement::from(1u64)
        )]
    );
}

#[test]
fn test_nullable_json_obj_filter_rejects_fragment_overflow() {
    let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::<
        TestOversizedWhereValue,
    >::from(None);
    let column = constants_str::X
        .repeat(crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN);
    let mut increment = crate::query_part_increment::QueryPartIncrement::from(0u64);
    assert!(matches!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &filter,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&column),
            crate::add_operator::AddOperator::from(false),
        ),
        Err(crate::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
}

#[test]
fn test_query_part_error_conversion_preserves_exact_length_details() {
    [(1usize, 0usize), (65_535usize, 4096usize), (1_048_577usize, 1usize)]
        .into_iter()
        .fold((), |(), (len, max)| {
            let source = crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong { len, max };
            let query_part_error = crate::query_part_error::QueryPartError::from(source);
            assert!(matches!(
                &query_part_error,
                crate::query_part_error::QueryPartError::StringWrapperTryFromString {
                    error: crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
                        len: actual_length,
                        max: maximum_length,
                    },
                    ..
                } if *actual_length == len && *maximum_length == max
            ));
            assert_eq!(
                to_err_string::to_err_string::ToErrString::to_err_string(&query_part_error).as_ref(),
                query_part_error.to_string(),
            );
        });
}

#[test]
fn test_query_part_error_formats_as_error_text() {
    let error = crate::query_part_error::QueryPartError::CheckedAdd {
        location: proc_macro_location_bang::location!(),
    };
    let error_text = to_err_string::to_err_string::ToErrString::to_err_string(&error);
    assert_eq!(error_text.as_ref(), error.to_string());
}

#[test]
fn test_nullable_json_obj_null_filter_preserves_counter_and_existing_arguments() {
    let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::<
        TestOversizedWhereValue,
    >::from(None);
    assert!([false, true].into_iter().all(|operator| {
        let mut increment = crate::query_part_increment::QueryPartIncrement::from(u64::MAX);
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &filter,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
            crate::add_operator::AddOperator::from(operator),
        )
        .is_ok_and(|part| {
            part.to_string() == format!("{} = '{}'", constants_str::SQL_NAMES_ID, stringify!(null))
                && increment.get() == u64::MAX
        })
    }));
    assert!(filter.as_ref().is_none());
    let query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(
        sqlx::query(constants_str::EMPTY).bind(17i64),
    );
    assert!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(filter, query).is_ok_and(
            |bound| {
                let mut sqlx_query = bound.into_inner();
                sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                    arguments.is_some_and(|bound_arguments| {
                        sqlx::Arguments::len(&bound_arguments) == 1usize
                    })
                })
            }
        )
    );
}

#[test]
fn test_nullable_json_obj_non_null_filter_preserves_values_and_query_errors() {
    let Ok(values) = crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
        crate::duplicate_candidates::DuplicateCandidates::from(vec![
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(1u64)),
            TestOversizedWhereValue(crate::query_part_increment::QueryPartIncrement::from(2u64)),
        ]),
    ) else {
        std::panic::panic_any(constants_str::PANIC_EDC94D17);
    };
    let filter =
        crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(
            Some(values.clone()),
        );
    assert_eq!(filter.as_ref(), Some(&values));
    let mut increment = crate::query_part_increment::QueryPartIncrement::from(7u64);
    assert!(matches!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &filter,
            &mut increment,
            crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
            crate::add_operator::AddOperator::from(false)
        ),
        Err(crate::query_part_error::QueryPartError::StringWrapperTryFromString { .. })
    ));
    assert_eq!(increment.get(), 7u64);
    assert_eq!(filter.into_option(), Some(values));
}

#[test]
fn test_nullable_json_obj_non_null_filter_delegates_argument_binding() {
    let values: crate::not_empty_unique_vec::NotEmptyUniqueVec<TestOversizedWhereValue> =
        crate::default_some_one_element::DefaultSomeOneElement::default_some_one_element();
    let filter =
        crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(
            Some(values),
        );
    let query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(
        sqlx::query(constants_str::EMPTY).bind(17i64),
    );
    assert!(
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(filter, query).is_ok_and(
            |bound| {
                let mut sqlx_query = bound.into_inner();
                sqlx::Execute::take_arguments(&mut sqlx_query).is_ok_and(|arguments| {
                    arguments.is_some_and(|bound_arguments| {
                        sqlx::Arguments::len(&bound_arguments) == 1usize
                    })
                })
            }
        )
    );
}

#[test]
fn test_nullable_json_obj_default_variants_preserve_non_null_values_and_diagnostics() {
    let variants = Vec::from(<crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter<TestOversizedWhereValue> as crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement>::all_variants_default_some_one_element());
    assert_eq!(variants.len(), 1usize);
    assert!(variants.first().is_some_and(|filter| {
        let diagnostic = to_err_string::to_err_string::ToErrString::to_err_string(filter);
        assert_eq!(diagnostic.as_ref(), format!("{filter:#?}"));
        filter.as_ref().is_some_and(|values| {
            values.as_slice()
                == [TestOversizedWhereValue(
                    crate::query_part_increment::QueryPartIncrement::from(1u64),
                )]
        })
    }));
    let null_filter =
        crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::<
            TestOversizedWhereValue,
        >::from(None);
    let null_diagnostic = to_err_string::to_err_string::ToErrString::to_err_string(&null_filter);
    assert_eq!(null_diagnostic.as_ref(), format!("{null_filter:#?}"));
}

#[test]
fn test_nullable_json_object_success_delegates_sql_progress_and_appended_arguments() {
    assert!(crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(
        crate::duplicate_candidates::DuplicateCandidates::from(vec![
            TestFallibleWhereValue::Accepted(crate::query_part_increment::QueryPartIncrement::from(1u64)),
            TestFallibleWhereValue::Accepted(crate::query_part_increment::QueryPartIncrement::from(2u64)),
        ]),
    ).is_ok_and(|values| {
        let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(Some(values));
        assert!([false, true].into_iter().all(|operator| {
            let mut increment = crate::query_part_increment::QueryPartIncrement::from(7u64);
            let expected = format!("{}{}{}{}", if operator { constants_str::AND_ALT } else { constants_str::EMPTY }, constants_str::X, constants_str::AND_ALT, constants_str::X);
            crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
                &filter, &mut increment,
                crate::sql_column_ref::SqlColumnRef::from(&constants_str::SQL_NAMES_ID),
                crate::add_operator::AddOperator::from(operator),
            ).is_ok_and(|fragment| fragment.as_ref() == expected && increment.get() == 9u64)
        }));
        let query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(sqlx::query(constants_str::TEST_READ_QUERY_BASE).bind(17i64));
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(filter, query).is_ok_and(|bound| {
            let mut sqlx_query = bound.into_inner();
            let arguments_result = sqlx::Execute::take_arguments(&mut sqlx_query);
            assert_eq!(sqlx::Execute::sql(sqlx_query).as_str(), constants_str::TEST_READ_QUERY_BASE);
            arguments_result.is_ok_and(|arguments| arguments.is_some_and(|bound_arguments| sqlx::Arguments::len(&bound_arguments) == 3usize))
        })
    }));
}

#[test]
fn test_nullable_json_object_binding_rejections_preserve_error_source() {
    assert!([
        vec![TestFallibleWhereValue::Rejected(crate::query_part_increment::QueryPartIncrement::from(1u64))],
        vec![TestFallibleWhereValue::Accepted(crate::query_part_increment::QueryPartIncrement::from(1u64)), TestFallibleWhereValue::Rejected(crate::query_part_increment::QueryPartIncrement::from(2u64))],
    ].into_iter().all(|candidates| {
        crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(crate::duplicate_candidates::DuplicateCandidates::from(candidates)).is_ok_and(|values| {
            let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(Some(values));
            let query = crate::sqlx_postgres_query::SqlxPostgresQuery::from(sqlx::query(constants_str::EMPTY).bind(17i64));
            crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(filter, query).is_err_and(|error| {
                std::error::Error::source(&error).and_then(std::error::Error::source)
                    .and_then(|source| source.downcast_ref::<std::io::Error>())
                    .is_some_and(|source| source.kind() == std::io::ErrorKind::InvalidData)
            })
        })
    }));
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(from = "crate::pagination_base::PaginationBase")]
struct TestSerializablePaginationFilter(crate::pagination_base::PaginationBase);

impl<'query_lt> crate::pg_type_where_filter::PgTypeWhereFilter<'query_lt>
    for TestSerializablePaginationFilter
{
    fn query_bind(
        self,
        sqlx_postgres_query: crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
    ) -> Result<
        crate::sqlx_postgres_query::SqlxPostgresQuery<'query_lt>,
        crate::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError,
    > {
        crate::pg_type_where_filter::PgTypeWhereFilter::query_bind(self.0, sqlx_postgres_query)
    }
    fn query_part(
        &self,
        increment: &mut dyn crate::query_part_increment_mut::QueryPartIncrementMut,
        sql_column_ref: crate::sql_column_ref::SqlColumnRef<'_>,
        add_operator: crate::add_operator::AddOperator,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::query_part_error::QueryPartError,
    > {
        crate::pg_type_where_filter::PgTypeWhereFilter::query_part(
            &self.0,
            increment,
            sql_column_ref,
            add_operator,
        )
    }
}
impl crate::all_enum_variants_array_default_some_one_element::AllEnumVariantsArrayDefaultSomeOneElement for TestSerializablePaginationFilter {
    fn all_variants_default_some_one_element() -> crate::all_enum_variants::AllEnumVariants<Self> {
        vec![Self::from(crate::pagination_base::PaginationBase::default())].into()
    }
}

#[test]
fn test_nullable_json_filter_serialization_preserves_null_and_ordered_values() {
    let null_filter =
        crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::<
            TestSerializablePaginationFilter,
        >::from(None);
    assert_eq!(
        serde_json::to_value(&null_filter).ok(),
        Some(serde_json::Value::Null)
    );
    assert!(
        serde_json::from_value::<
            crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter<
                TestSerializablePaginationFilter,
            >,
        >(serde_json::Value::Null)
        .is_ok_and(|filter| filter.as_ref().is_none())
    );
    let first = crate::pagination_base::PaginationBase::default();
    let second = crate::pagination_base::PaginationBase::new_unchecked(2i32, 3i32);
    let expected_values = [
        TestSerializablePaginationFilter::from(first),
        TestSerializablePaginationFilter::from(second),
    ];
    assert!(crate::not_empty_unique_vec::NotEmptyUniqueVec::try_new(crate::duplicate_candidates::DuplicateCandidates::from(expected_values.to_vec())).is_ok_and(|values| {
        let filter = crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter::from(Some(values));
        let expected_json = serde_json::json!([first, second]);
        assert_eq!(serde_json::to_value(&filter).ok().as_ref(), Some(&expected_json));
        serde_json::from_value::<crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter<TestSerializablePaginationFilter>>(expected_json)
            .is_ok_and(|decoded| decoded.into_option().is_some_and(|decoded_values| decoded_values.as_slice() == expected_values))
    }));
}

#[test]
fn test_nullable_json_filter_deserialization_rejects_invalid_non_null_values() {
    let value = crate::pagination_base::PaginationBase::default();
    assert!(
        [
            serde_json::json!([]),
            serde_json::json!([value, value]),
            serde_json::json!([null]),
            serde_json::json!([true]),
            serde_json::json!({}),
            serde_json::json!(constants_str::X),
        ]
        .into_iter()
        .all(|json| {
            serde_json::from_value::<
                crate::nullable_json_obj_pg_type_where_filter::NullableJsonObjPgTypeWhereFilter<
                    TestSerializablePaginationFilter,
                >,
            >(json)
            .is_err_and(|error| error.is_data())
        })
    );
}
