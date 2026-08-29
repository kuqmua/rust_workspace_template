// The owner module retains lint-sensitive semantics from the original implementation.
#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypeNotPrimaryKey {
    type PgType: crate::pg_type::PgType;
    type Create: crate::domain_types::CreateAlias + crate::domain_types::SqlxEncodePgSqlxTypePgAlias;
}
