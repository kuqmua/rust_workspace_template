#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::AsRefStr,
    newtype::TryFrom,
)]
#[try_from(validator = SqlIdentifier::validate)]
pub struct SqlIdentifier(String);
impl SqlIdentifier {
    #[allow(clippy::single_call_fn)] // derive-generated TryFrom owns the single validator call
    fn validate(value: &str) -> Result<(), SqlIdentifierError> {
        if value.len() > 128usize {
            return Err(SqlIdentifierError::Invalid);
        }
        let mut bytes = value.bytes();
        let first = bytes.next().ok_or(SqlIdentifierError::Empty)?;
        if !(first.is_ascii_alphabetic() || first == b'_')
            || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(SqlIdentifierError::Invalid);
        }
        Ok(())
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum SqlIdentifierError {
    #[error("SQL identifier is empty")]
    Empty,
    #[error("SQL identifier contains unsupported characters")]
    Invalid,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlQualifiedIdentifier {
    schema: SqlIdentifier,
    table: SqlIdentifier,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
struct SqlIdentifierListText(String);
impl TryFrom<String> for SqlIdentifierListText {
    type Error = crate::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(crate::PgCrudStringWrapperTryFromStringError::TooLong {
                len: value.len(),
                max: crate::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlIdentifiers(SqlIdentifierListText);
impl TryFrom<Vec<SqlIdentifier>> for SqlIdentifiers {
    type Error = crate::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: Vec<SqlIdentifier>) -> Result<Self, Self::Error> {
        if value.len() > bounded_types::domain_types::COLLECTION_MAX_LEN {
            return Err(crate::PgCrudStringWrapperTryFromStringError::TooLong {
                len: value.len(),
                max: bounded_types::domain_types::COLLECTION_MAX_LEN,
            });
        }
        let identifiers_len = value.iter().fold(constants_usize::ZERO, |len, identifier| {
            len.saturating_add(identifier.as_ref().len())
        });
        let separators_len = value
            .len()
            .saturating_sub(constants_usize::ONE)
            .saturating_mul(constants_str::TEXT_ALT_6.len());
        let mut text = String::with_capacity(identifiers_len.saturating_add(separators_len));
        value.iter().enumerate().for_each(|(idx, identifier)| {
            if idx != constants_usize::ZERO {
                text.push_str(constants_str::TEXT_ALT_6);
            }
            text.push_str(identifier.as_ref());
        });
        SqlIdentifierListText::try_from(text).map(Self)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct SqlQueryText(String);
impl From<crate::PgCrudStringWrapperTryFromStringError> for SqlQueryText {
    fn from(value: crate::PgCrudStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for SqlQueryText {
    type Error = crate::PgCrudStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::PG_CRUD_STRING_WRAPPER_MAX_LEN {
            Err(crate::PgCrudStringWrapperTryFromStringError::TooLong {
                len: value.len(),
                max: crate::PG_CRUD_STRING_WRAPPER_MAX_LEN,
            })
        } else {
            Ok(Self(value))
        }
    }
}
impl SqlQualifiedIdentifier {
    #[must_use]
    pub const fn new(schema: SqlIdentifier, table: SqlIdentifier) -> Self {
        Self { schema, table }
    }
    fn push_to(&self, output: &mut SqlQueryText) {
        output.0.push_str(self.schema.as_ref());
        output.0.push('.');
        output.0.push_str(self.table.as_ref());
    }
}
impl std::fmt::Display for SqlQualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.schema.as_ref())?;
        f.write_str(constants_str::DOT)?;
        f.write_str(self.table.as_ref())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlSelectBuilder {
    columns: SqlIdentifiers,
    table: SqlQualifiedIdentifier,
}
impl SqlSelectBuilder {
    #[must_use]
    pub fn build(&self) -> crate::QueryPartFragment {
        let fixed_len = constants_str::SELECT
            .len()
            .saturating_add(constants_str::FROM.len())
            .saturating_add(self.table.schema.as_ref().len())
            .saturating_add(constants_str::DOT.len())
            .saturating_add(self.table.table.as_ref().len());
        let columns = self.columns.0.0.as_str();
        let capacity = fixed_len.saturating_add(columns.len());
        let mut query = SqlQueryText::try_from(String::with_capacity(capacity))
            .unwrap_or_else(SqlQueryText::from);
        query.0.push_str(constants_str::SELECT);
        query.0.push_str(columns);
        query.0.push_str(constants_str::FROM);
        self.table.push_to(&mut query);
        crate::QueryPartFragment::try_from(query.0).unwrap_or_else(crate::QueryPartFragment::from)
    }
    #[must_use]
    pub const fn new(table: SqlQualifiedIdentifier, columns: SqlIdentifiers) -> Self {
        Self { columns, table }
    }
}
#[cfg(test)]
mod tests {
    fn identifier(value: &str) -> super::SqlIdentifier {
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
        let builder = super::SqlSelectBuilder::new(
            super::SqlQualifiedIdentifier::new(
                identifier(constants_str::PUBLIC),
                identifier(constants_str::USERS_ALT),
            ),
            super::SqlIdentifiers::try_from(vec![
                identifier(constants_str::SQL_NAMES_ID),
                identifier(constants_str::LOGIN),
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
