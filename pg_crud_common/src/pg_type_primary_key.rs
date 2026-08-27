// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypePrimaryKey {
    type PgType: crate::domain_types::PgType;
    type TableType: crate::domain_types::TableTypeAlias + PartialOrd;

    fn read_ids_into_table_type(
        v: <Self::PgType as crate::domain_types::PgType>::ReadIds,
    ) -> <Self::PgType as crate::domain_types::PgType>::TableType;

    fn read_ids_into_read(
        v: <Self::PgType as crate::domain_types::PgType>::ReadIds,
    ) -> <Self::PgType as crate::domain_types::PgType>::Read;

    fn read_ids_into_update(
        v: <Self::PgType as crate::domain_types::PgType>::ReadIds,
    ) -> <Self::PgType as crate::domain_types::PgType>::Update;

    fn read_into_table_type(
        v: <Self::PgType as crate::domain_types::PgType>::Read,
    ) -> <Self::PgType as crate::domain_types::PgType>::TableType;
}
