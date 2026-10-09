fn test_assert_generated_numeric_comparison_cases<GeneratedPgType>()
where
    GeneratedPgType: pg_crud_common::pg_type::PgType<
            ReadIds = pg_crud_common::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds,
        > + pg_crud_common::pg_type_test_cases::PgTypeTestCases<PgType = GeneratedPgType>,
{
    assert!(GeneratedPgType::pg_type_optional_vec_where_greater_than_test().is_some_and(|cases| {
        cases.as_slice().len() == 9usize
            && cases.into_vec().into_iter().enumerate().all(|(index, case)| {
                let (threshold, create, variant) = case.into_parts();
                let expected_variant = if index < 3usize {
                    pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::GreaterThan
                } else if index < 6usize {
                    pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::NotGreaterThan
                } else {
                    pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::EqNotGreaterThan
                };
                let expected_order = match variant {
                    pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::GreaterThan => std::cmp::Ordering::Greater,
                    pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::NotGreaterThan => std::cmp::Ordering::Less,
                    pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::EqNotGreaterThan => std::cmp::Ordering::Equal,
                };
                let ordered = serde_json::to_value(create).is_ok_and(|create_json| {
                    serde_json::to_value(&threshold).is_ok_and(|threshold_json| {
                        let actual_order = match (create_json.as_i64(), threshold_json.as_i64()) {
                            (Some(create_integer), Some(threshold_integer)) => Some(create_integer.cmp(&threshold_integer)),
                            _ => create_json.as_f64().zip(threshold_json.as_f64()).and_then(|(create_float, threshold_float)| create_float.partial_cmp(&threshold_float)),
                        };
                        actual_order == Some(expected_order)
                    })
                });
                variant == expected_variant && ordered
                    && GeneratedPgType::read_ids_and_table_type_into_pg_type_optional_where_greater_than(
                        variant,
                        pg_crud_common::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds::default(),
                        threshold,
                    ).is_some()

            })
    }));
}

#[test]
fn test_generated_non_null_numeric_comparisons_preserve_fixture_order_and_build_filters() {
    test_assert_generated_numeric_comparison_cases::<crate::generate_pg_types_mod::I16AsNonNullInt2>(
    );
    test_assert_generated_numeric_comparison_cases::<crate::generate_pg_types_mod::I32AsNonNullInt4>(
    );
    test_assert_generated_numeric_comparison_cases::<crate::generate_pg_types_mod::I64AsNonNullInt8>(
    );
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::F32AsNonNullFloat4,
    >();
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::F64AsNonNullFloat8,
    >();
}

#[test]
fn test_generated_nullable_numeric_comparisons_preserve_fixture_order_and_build_filters() {
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::OptionalI16AsNullableInt2,
    >();
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::OptionalI32AsNullableInt4,
    >();
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::OptionalI64AsNullableInt8,
    >();
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::OptionalF32AsNullableFloat4,
    >();
    test_assert_generated_numeric_comparison_cases::<
        crate::generate_pg_types_mod::OptionalF64AsNullableFloat8,
    >();
}

#[test]
fn test_generated_nullable_comparison_with_null_threshold_returns_no_filter() {
    assert!([
        pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::GreaterThan,
        pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::NotGreaterThan,
        pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::EqNotGreaterThan,
    ].into_iter().all(|variant| {
        serde_json::from_value::<<crate::generate_pg_types_mod::OptionalI16AsNullableInt2 as pg_crud_common::pg_type::PgType>::TableType>(serde_json::Value::Null)
            .is_ok_and(|threshold| <crate::generate_pg_types_mod::OptionalI16AsNullableInt2 as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::read_ids_and_table_type_into_pg_type_optional_where_greater_than(variant, pg_crud_common::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds::default(), threshold).is_none())
    }));
}

#[test]
fn test_boolean_case_providers_omit_unsupported_array_and_ordering_hooks() {
    fn assert_boolean_provider_omits_unsupported_hooks<GeneratedPgType>()
    where
        GeneratedPgType: pg_crud_common::pg_type::PgType<
                ReadIds = pg_crud_common::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds,
            > + pg_crud_common::pg_type_test_cases::PgTypeTestCases<PgType = GeneratedPgType>,
    {
        let read_ids = || {
            pg_crud_common::non_primary_key_pg_type_read_ids::NonPrimaryKeyPgTypeReadIds::default()
        };
        let create = || {
            <<GeneratedPgType as pg_crud_common::pg_type::PgType>::Create as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element()
        };
        assert!(<GeneratedPgType as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::read_ids_and_create_into_optional_vec_where_eq_to_field(read_ids(), create()).is_none());
        assert!(<GeneratedPgType as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::create_into_pg_type_optional_vec_where_dimension_one_eq(create()).is_none());
        assert!(<GeneratedPgType as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::pg_type_optional_vec_where_greater_than_test().is_none());
        assert!([
            pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::GreaterThan,
            pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::NotGreaterThan,
            pg_crud_common::pg_type_greater_than_variant::PgTypeGreaterThanVariant::EqNotGreaterThan,
        ].into_iter().all(|variant| <GeneratedPgType as pg_crud_common::pg_type_test_cases::PgTypeTestCases>::read_ids_and_table_type_into_pg_type_optional_where_greater_than(
            variant, read_ids(), <<GeneratedPgType as pg_crud_common::pg_type::PgType>::TableType as pg_crud_common::default_some_one_element::DefaultSomeOneElement>::default_some_one_element(),
        ).is_none()));
    }
    assert_boolean_provider_omits_unsupported_hooks::<
        crate::generate_pg_types_mod::BoolAsNonNullBool,
    >();
    assert_boolean_provider_omits_unsupported_hooks::<
        crate::generate_pg_types_mod::OptionalBoolAsNullableBool,
    >();
}
