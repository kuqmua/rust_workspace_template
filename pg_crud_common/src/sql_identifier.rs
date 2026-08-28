#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
)]
pub struct SqlIdentifier(String);
impl TryFrom<String> for SqlIdentifier {
    type Error = crate::domain_types::SqlIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 128usize {
            return Err(crate::domain_types::SqlIdentifierError::Invalid);
        }
        let mut bytes = value.bytes();
        let first = bytes
            .next()
            .ok_or(crate::domain_types::SqlIdentifierError::Empty)?;
        if !(first.is_ascii_alphabetic() || first == b'_')
            || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(crate::domain_types::SqlIdentifierError::Invalid);
        }
        Ok(Self(value))
    }
}
#[cfg(test)]
mod tests {
    fn sql_identifier_fixture(value: &str) -> super::SqlIdentifier {
        super::SqlIdentifier::try_from(value.to_owned())
            .expect("940eb924 identifier invariant must hold")
    }
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository source policy requires iterator methods instead of for loops"
    )]
    fn sql_identifier_uses_restricted_ascii_grammar() {
        [
            constants_str::TABLE_ALT,
            constants_str::TABLE,
            constants_str::TABLE_2,
        ]
        .into_iter()
        .for_each(|value| {
            let _identifier = super::SqlIdentifier::try_from(value.to_owned()).expect(
                "326a4da9 sql_identifier_uses_restricted_ascii_grammar invariant must hold",
            );
        });
        [
            constants_str::PG_CRUD_EMPTY_SQL_SUFFIX,
            constants_str::VALUE_2TABLE,
            constants_str::TABLE_NAME,
            constants_str::NON_ASCII_U_E9,
            constants_str::TABLE_NAME_ALT,
        ]
        .into_iter()
        .for_each(|value| {
            let _error = super::SqlIdentifier::try_from(value.to_owned())
                .expect_err(constants_str::F698FD6D);
        });
    }
    #[test]
    fn query_builder_accepts_only_validated_identifiers() {
        let builder = crate::domain_types::SqlSelectBuilder::new(
            crate::domain_types::SqlQualifiedIdentifier::new(
                sql_identifier_fixture(constants_str::PUBLIC),
                sql_identifier_fixture(constants_str::USERS_ALT),
            ),
            crate::domain_types::SqlIdentifiers::try_from(vec![
                sql_identifier_fixture(constants_str::SQL_NAMES_ID),
                sql_identifier_fixture(constants_str::LOGIN),
            ])
            .expect(
                "c4cf723e query_builder_accepts_only_validated_identifiers invariant must hold",
            ),
        );
        let first = builder.build();
        let second = builder.build();
        assert_eq!(first.into_inner(), "SELECT id, login FROM public.users");
        assert_eq!(second.into_inner(), "SELECT id, login FROM public.users");
    }
    #[test]
    fn benchmark_black_box_dependency_is_available() {
        assert_ne!(size_of::<criterion::Criterion>(), constants_usize::ZERO);
    }
}
