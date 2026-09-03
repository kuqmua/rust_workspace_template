#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type test cases keeps declaration order aligned with generated layout or processing flow"
)]
#[allow(
    unused_variables,
    reason = "default test-contract hooks preserve repository type-based parameter names"
)]
#[cfg(feature = "test-utils")]
pub trait PgTypeTestCases {
    type PgType: crate::pg_type::PgType;
    type Select: crate::domain_types::SelectAlias
        + crate::default_some_one_element_max_page_size::DefaultSomeOneElementMaxPageSize;
    #[must_use]
    fn optional_vec_create() -> Option<Vec<<Self::PgType as crate::pg_type::PgType>::Create>> {
        None
    }
    fn read_ids_to_2_dimensions_vec_read_inner(
        read_ids: &<Self::PgType as crate::pg_type::PgType>::ReadIds,
    ) -> Vec<Vec<<Self::PgType as crate::pg_type::PgType>::ReadInner>>;
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        read_inner: <Self::PgType as crate::pg_type::PgType>::ReadInner,
    ) -> <Self::PgType as crate::pg_type::PgType>::Read;
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        read_inner: <Self::PgType as crate::pg_type::PgType>::ReadInner,
    ) -> <Self::PgType as crate::pg_type::PgType>::Update;
    fn update_to_read_ids(
        update: &<Self::PgType as crate::pg_type::PgType>::Update,
    ) -> <Self::PgType as crate::pg_type::PgType>::ReadIds;
    fn read_ids_to_optional_explicit_value_read_default_some_one_element(
        read_ids: &<Self::PgType as crate::pg_type::PgType>::ReadIds,
    ) -> Option<crate::explicit_value::ExplicitValue<<Self::PgType as crate::pg_type::PgType>::Read>>
    {
        None
    }
    fn previous_read_and_optional_update_into_read(
        read: <Self::PgType as crate::pg_type::PgType>::Read,
        option: Option<<Self::PgType as crate::pg_type::PgType>::Update>,
    ) -> <Self::PgType as crate::pg_type::PgType>::Read;
    fn read_ids_and_create_into_read(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> <Self::PgType as crate::pg_type::PgType>::Read;
    fn read_ids_and_create_into_optional_explicit_value_read(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> Option<crate::explicit_value::ExplicitValue<<Self::PgType as crate::pg_type::PgType>::Read>>
    {
        None
    }
    fn read_ids_and_create_into_table_type(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> <Self::PgType as crate::pg_type::PgType>::TableType;
    fn read_ids_and_create_into_where_eq(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> <Self::PgType as crate::pg_type::PgType>::Where;
    fn read_ids_and_create_into_vec_where_eq_using_fields(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> crate::not_empty_unique_vec::NotEmptyUniqueVec<
        <Self::PgType as crate::pg_type::PgType>::Where,
    >;
    fn read_ids_and_create_into_optional_vec_where_eq_to_field(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> Option<
        crate::not_empty_unique_vec::NotEmptyUniqueVec<
            <Self::PgType as crate::pg_type::PgType>::Where,
        >,
    > {
        None
    }
    fn create_into_pg_type_optional_vec_where_dimension_one_eq(
        create: <Self::PgType as crate::pg_type::PgType>::Create,
    ) -> Option<
        crate::not_empty_unique_vec::NotEmptyUniqueVec<
            <Self::PgType as crate::pg_type::PgType>::Where,
        >,
    > {
        None
    }
    #[must_use]
    fn pg_type_optional_vec_where_greater_than_test() -> Option<
        crate::not_empty_unique_vec::NotEmptyUniqueVec<
            crate::pg_type_greater_than_test::PgTypeGreaterThanTest<Self::PgType>,
        >,
    > {
        None
    }
    fn read_ids_and_table_type_into_pg_type_optional_where_greater_than(
        pg_type_greater_than_variant: crate::pg_type_greater_than_variant::PgTypeGreaterThanVariant,
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
        table_type: <Self::PgType as crate::pg_type::PgType>::TableType,
    ) -> Option<<Self::PgType as crate::pg_type::PgType>::Where> {
        None
    }
}
