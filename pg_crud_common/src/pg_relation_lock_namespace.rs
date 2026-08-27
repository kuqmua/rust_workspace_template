#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgRelationLockNamespace(pub(super) String);

impl TryFrom<String> for PgRelationLockNamespace {
    type Error = crate::domain_types::PgRelationLockError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 128usize {
            return Err(crate::domain_types::PgRelationLockError::InvalidNamespace);
        }
        text_policy::domain_types::validate_url_safe_token_part(
            text_policy::domain_types::UrlSafeTokenPartRef::from(value.as_str()),
            text_policy::domain_types::UrlSafeTokenPartMaximumBytes::from(128usize),
        )
        .map_err(|_error| crate::domain_types::PgRelationLockError::InvalidNamespace)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn namespace_rejects_sql_syntax() {
        assert_eq!(
            super::PgRelationLockNamespace::try_from(String::from(
                constants_str::TEST_SQL_INJECTION
            )),
            Err(crate::domain_types::PgRelationLockError::InvalidNamespace)
        );
    }
}
