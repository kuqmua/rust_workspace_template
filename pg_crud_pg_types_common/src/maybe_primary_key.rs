#[must_use]
pub fn maybe_primary_key<V>(v: V) -> impl std::fmt::Display
where
    V: Into<crate::is_primary_key::IsPrimaryKey>,
{
    if bool::from(v.into()) {
        constants_str::PRIMARY_KEY
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primary_key_suffix_matches_the_typed_flag() {
        assert_eq!(
            super::maybe_primary_key(crate::is_primary_key::IsPrimaryKey::from(true)).to_string(),
            constants_str::PRIMARY_KEY
        );
        assert_eq!(
            super::maybe_primary_key(crate::is_primary_key::IsPrimaryKey::from(false)).to_string(),
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
        );
        assert_eq!(
            super::maybe_primary_key(pg_crud_common::domain_types::IsPrimaryKey::from(true))
                .to_string(),
            constants_str::PRIMARY_KEY
        );
    }
}
