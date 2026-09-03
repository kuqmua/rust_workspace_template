#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "pg type not primary key keeps declaration order aligned with generated layout or processing flow"
)]
pub trait PgTypeNotPrimaryKey {
    type PgType: crate::pg_type::PgType;
    type Create: crate::domain_types::CreateAlias + crate::domain_types::SqlxEncodePgSqlxTypePgAlias;
}
