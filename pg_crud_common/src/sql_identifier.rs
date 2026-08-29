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
    type Error = crate::sql_identifier_error::SqlIdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 128usize {
            return Err(crate::sql_identifier_error::SqlIdentifierError::Invalid);
        }
        let mut bytes = value.bytes();
        let first = bytes
            .next()
            .ok_or(crate::sql_identifier_error::SqlIdentifierError::Empty)?;
        if !(first.is_ascii_alphabetic() || first == b'_')
            || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(crate::sql_identifier_error::SqlIdentifierError::Invalid);
        }
        Ok(Self(value))
    }
}
#[cfg(test)]
mod tests {
    fn sql_identifier_fixture(value: &str) -> crate::sql_identifier::SqlIdentifier {
        crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
            .expect("940eb924 identifier invariant must hold")
    }
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository source policy requires iterator methods instead of for loops"
    )]
    fn sql_identifier_uses_restricted_ascii_grammar() {
        [
            constants_str::catalog::TABLE_ALT,
            constants_str::catalog::TABLE,
            constants_str::catalog::TABLE_2,
        ]
        .into_iter()
        .for_each(|value| {
            let _identifier = crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
                .expect(
                    "326a4da9 sql_identifier_uses_restricted_ascii_grammar invariant must hold",
                );
        });
        [
            constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
            constants_str::catalog::VALUE_2TABLE,
            constants_str::catalog::TABLE_NAME,
            constants_str::catalog::NON_ASCII_U_E9,
            constants_str::catalog::TABLE_NAME_ALT,
        ]
        .into_iter()
        .for_each(|value| {
            let _error = crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
                .expect_err(constants_str::catalog::F698FD6D);
        });
    }
    #[test]
    fn query_builder_accepts_only_validated_identifiers() {
        let builder = crate::sql_select_builder::SqlSelectBuilder::new(
            crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                sql_identifier_fixture(constants_str::catalog::PUBLIC),
                sql_identifier_fixture(constants_str::catalog::USERS_ALT),
            ),
            crate::sql_identifiers::SqlIdentifiers::try_from(vec![
                sql_identifier_fixture(constants_str::catalog::SQL_NAMES_ID),
                sql_identifier_fixture(constants_str::catalog::LOGIN),
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
