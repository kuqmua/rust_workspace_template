#[derive(
    proc_macro_getters::Getters,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct PgRelationLockNamespace(
    bounded_types::bounded_string::BoundedString<0usize, 128usize, false>,
);

impl TryFrom<String> for PgRelationLockNamespace {
    type Error = crate::pg_relation_lock_error::PgRelationLockError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 128usize {
            return Err(crate::pg_relation_lock_error::PgRelationLockError::InvalidNamespace);
        }
        text_policy::validate_url_safe_token_part::validate_url_safe_token_part(
            text_policy::url_safe_token_part_ref::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::url_safe_token_part_maximum_bytes::UrlSafeTokenPartMaximumBytes::from(
                128usize,
            ),
        )
        .map_err(|_error| crate::pg_relation_lock_error::PgRelationLockError::InvalidNamespace)?;
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::InvalidNamespace,
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_relation_namespace_accepts_token_boundaries_and_rejects_invalid_bytes() {
        assert!(
            [
                String::from(constants_str::X),
                constants_str::X.repeat(128usize),
                ['a', 'Z', '0', '_', '-'].into_iter().collect(),
            ]
            .into_iter()
            .all(|value| {
                crate::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(value).is_ok()
            })
        );
        assert!(
            [
                String::new(),
                constants_str::X.repeat(129usize),
                ' '.to_string(),
                '/'.to_string(),
                '\u{00e9}'.to_string(),
            ]
            .into_iter()
            .all(|value| {
                crate::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(value)
                    == Err(crate::pg_relation_lock_error::PgRelationLockError::InvalidNamespace)
            })
        );
    }

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
