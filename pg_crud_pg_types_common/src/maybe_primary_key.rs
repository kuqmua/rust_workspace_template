#[must_use]
pub fn maybe_primary_key<V>(v: V) -> impl std::fmt::Display
where
    V: Into<crate::domain_types::IsPrimaryKey>,
{
    if bool::from(v.into()) {
        constants_str::PRIMARY_KEY
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    }
}
