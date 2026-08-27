#[allow(clippy::arbitrary_source_item_ordering)]
pub trait PgTypeNotPrimaryKey {
    type PgType: crate::domain_types::PgType;
    type Create: crate::domain_types::CreateAlias + crate::domain_types::SqlxEncodePgSqlxTypePgAlias;
}
