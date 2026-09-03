#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
pub trait PgTypePrimaryKey {
    type PgType: crate::pg_type::PgType;
    type TableType: crate::domain_types::TableTypeAlias + PartialOrd;

    fn read_ids_into_table_type(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
    ) -> <Self::PgType as crate::pg_type::PgType>::TableType;

    fn read_ids_into_read(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
    ) -> <Self::PgType as crate::pg_type::PgType>::Read;

    fn read_ids_into_update(
        read_ids: <Self::PgType as crate::pg_type::PgType>::ReadIds,
    ) -> <Self::PgType as crate::pg_type::PgType>::Update;

    fn read_into_table_type(
        read: <Self::PgType as crate::pg_type::PgType>::Read,
    ) -> <Self::PgType as crate::pg_type::PgType>::TableType;
}
