#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct PgRelationLockNamespace(String);

impl TryFrom<String> for PgRelationLockNamespace {
    type Error = crate::pg_relation_lock_error::PgRelationLockError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > 128usize {
            return Err(crate::pg_relation_lock_error::PgRelationLockError::InvalidNamespace);
        }
        text_policy::validate_url_safe_token_part::validate_url_safe_token_part(
            text_policy::url_safe_token_part_ref::UrlSafeTokenPartRef::from(string.as_str()),
            text_policy::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                128usize,
            ),
        )
        .map_err(|_error| crate::pg_relation_lock_error::PgRelationLockError::InvalidNamespace)?;
        Ok(Self(string))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_namespace_rejects_sql_syntax() {
        assert_eq!(
            crate::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(String::from(
                constants_str::TEST_SQL_INJECTION
            )),
            Err(crate::pg_relation_lock_error::PgRelationLockError::InvalidNamespace)
        );
    }
}
