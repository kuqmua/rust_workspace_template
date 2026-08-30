#[must_use]
pub fn maybe_primary_key<V>(v: V) -> impl std::fmt::Display
where
    V: Into<crate::is_primary_key::IsPrimaryKey>,
{
    if bool::from(v.into()) {
        constants_str::catalog::PRIMARY_KEY
    } else {
        constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primary_key_suffix_matches_the_typed_flag() {
        assert_eq!(
            crate::maybe_primary_key::maybe_primary_key(crate::is_primary_key::IsPrimaryKey::from(
                true
            ))
            .to_string(),
            constants_str::catalog::PRIMARY_KEY
        );
        assert_eq!(
            crate::maybe_primary_key::maybe_primary_key(crate::is_primary_key::IsPrimaryKey::from(
                false
            ))
            .to_string(),
            constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX
        );
        assert_eq!(
            crate::maybe_primary_key::maybe_primary_key(
                pg_crud_common::pg_is_primary_key::PgIsPrimaryKey::from(true)
            )
            .to_string(),
            constants_str::catalog::PRIMARY_KEY
        );
    }
}
