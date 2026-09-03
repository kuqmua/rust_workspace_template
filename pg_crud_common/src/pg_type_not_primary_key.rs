#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "lint suppression is required here"
)]
pub trait PgTypeNotPrimaryKey {
    type PgType: crate::pg_type::PgType;
    type Create: crate::domain_types::CreateAlias + crate::domain_types::SqlxEncodePgSqlxTypePgAlias;
}
