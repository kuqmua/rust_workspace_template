#[cfg(feature = "test-utils")]
use crate::domain_types::SelectAlias;
#[cfg(feature = "test-utils")]
use crate::{
    DefaultSomeOneElementMaxPageSize, NotEmptyUniqueVec, PgType, PgTypeGreaterThanTest,
    PgTypeGreaterThanVariant, V,
};

// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
#[cfg(feature = "test-utils")]
pub trait PgTypeTestCases {
    type PgType: PgType;
    type Select: SelectAlias + DefaultSomeOneElementMaxPageSize;
    #[must_use]
    fn optional_vec_create() -> Option<Vec<<Self::PgType as PgType>::Create>> {
        None
    }
    fn read_ids_to_2_dimensions_vec_read_inner(
        read_ids: &<Self::PgType as PgType>::ReadIds,
    ) -> Vec<Vec<<Self::PgType as PgType>::ReadInner>>;
    fn read_inner_into_read_with_new_or_try_new_unwraped(
        v: <Self::PgType as PgType>::ReadInner,
    ) -> <Self::PgType as PgType>::Read;
    fn read_inner_into_update_with_new_or_try_new_unwraped(
        v: <Self::PgType as PgType>::ReadInner,
    ) -> <Self::PgType as PgType>::Update;
    fn update_to_read_ids(
        v: &<Self::PgType as PgType>::Update,
    ) -> <Self::PgType as PgType>::ReadIds;
    fn read_ids_to_optional_v_read_default_some_one_element(
        _v: &<Self::PgType as PgType>::ReadIds,
    ) -> Option<V<<Self::PgType as PgType>::Read>> {
        None
    }
    fn previous_read_and_optional_update_into_read(
        read: <Self::PgType as PgType>::Read,
        optional_update: Option<<Self::PgType as PgType>::Update>,
    ) -> <Self::PgType as PgType>::Read;
    fn read_ids_and_create_into_read(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> <Self::PgType as PgType>::Read;
    fn read_ids_and_create_into_optional_v_read(
        _read_ids: <Self::PgType as PgType>::ReadIds,
        _create: <Self::PgType as PgType>::Create,
    ) -> Option<V<<Self::PgType as PgType>::Read>> {
        None
    }
    fn read_ids_and_create_into_table_type(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> <Self::PgType as PgType>::TableType;
    fn read_ids_and_create_into_where_eq(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> <Self::PgType as PgType>::Where;
    fn read_ids_and_create_into_vec_where_eq_using_fields(
        read_ids: <Self::PgType as PgType>::ReadIds,
        create: <Self::PgType as PgType>::Create,
    ) -> NotEmptyUniqueVec<<Self::PgType as PgType>::Where>;
    fn read_ids_and_create_into_optional_vec_where_eq_to_field(
        _read_ids: <Self::PgType as PgType>::ReadIds,
        _create: <Self::PgType as PgType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PgType as PgType>::Where>> {
        None
    }
    fn create_into_pg_type_optional_vec_where_dimension_one_eq(
        _create: <Self::PgType as PgType>::Create,
    ) -> Option<NotEmptyUniqueVec<<Self::PgType as PgType>::Where>> {
        None
    }
    #[must_use]
    fn pg_type_optional_vec_where_greater_than_test()
    -> Option<NotEmptyUniqueVec<PgTypeGreaterThanTest<Self::PgType>>> {
        None
    }
    fn read_ids_and_table_type_into_pg_type_optional_where_greater_than(
        _greater_than_variant: PgTypeGreaterThanVariant,
        _read_ids: <Self::PgType as PgType>::ReadIds,
        _table_type: <Self::PgType as PgType>::TableType,
    ) -> Option<<Self::PgType as PgType>::Where> {
        None
    }
}
