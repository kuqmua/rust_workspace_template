#[derive(
    optml::Optml, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, newtype::AsRefStr, newtype::TryFrom,
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
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SqlIdentifierError {
    #[error("SQL identifier is empty")]
    Empty,
    #[error("SQL identifier contains unsupported characters")]
    Invalid,
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct SqlQualifiedIdentifier {
    schema: SqlIdentifier,
    table: SqlIdentifier,
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
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
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
enum SqlIdentifierListTextState {
    Text(SqlIdentifierListText),
    TooLong(crate::PgCrudStringWrapperTryFromStringError),
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct SqlIdentifiers(SqlIdentifierListTextState);
impl From<Vec<SqlIdentifier>> for SqlIdentifiers {
    fn from(value: Vec<SqlIdentifier>) -> Self {
        let identifiers_len = value.iter().fold(0usize, |len, identifier| {
            len.saturating_add(identifier.as_ref().len())
        });
        let separators_len = value
            .len()
            .saturating_sub(1usize)
            .saturating_mul(str_constants::TEXT_ALT_6.len());
        let mut text = String::with_capacity(identifiers_len.saturating_add(separators_len));
        value.iter().enumerate().for_each(|(idx, identifier)| {
            if idx != 0usize {
                text.push_str(str_constants::TEXT_ALT_6);
            }
            text.push_str(identifier.as_ref());
        });
        Self(match SqlIdentifierListText::try_from(text) {
            Ok(list_text) => SqlIdentifierListTextState::Text(list_text),
            Err(error) => SqlIdentifierListTextState::TooLong(error),
        })
    }
}
#[derive(optml::Optml, Debug)]
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
        f.write_str(str_constants::DOT)?;
        f.write_str(self.table.as_ref())
    }
}
#[derive(optml::Optml, Clone, Debug, Eq, PartialEq)]
pub struct SqlSelectBuilder {
    columns: SqlIdentifiers,
    table: SqlQualifiedIdentifier,
}
impl SqlSelectBuilder {
    #[must_use]
    pub fn build(&self) -> crate::QueryPartFragment {
        let fixed_len = str_constants::SELECT
            .len()
            .saturating_add(str_constants::FROM.len())
            .saturating_add(self.table.schema.as_ref().len())
            .saturating_add(str_constants::DOT.len())
            .saturating_add(self.table.table.as_ref().len());
        let columns = match &self.columns.0 {
            SqlIdentifierListTextState::Text(text) => text.0.as_str(),
            SqlIdentifierListTextState::TooLong(
                crate::PgCrudStringWrapperTryFromStringError::TooLong { len, max },
            ) => {
                return crate::QueryPartFragment::from(
                    crate::PgCrudStringWrapperTryFromStringError::TooLong {
                        len: fixed_len.saturating_add(*len),
                        max: *max,
                    },
                );
            }
        };
        let capacity = fixed_len.saturating_add(columns.len());
        let mut query = SqlQueryText::try_from(String::with_capacity(capacity))
            .unwrap_or_else(SqlQueryText::from);
        query.0.push_str(str_constants::SELECT);
        query.0.push_str(columns);
        query.0.push_str(str_constants::FROM);
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
            str_constants::TABLE_ALT,
            str_constants::TABLE,
            str_constants::TABLE_2,
        ]
        .into_iter()
        .for_each(|value| {
            let _identifier = super::SqlIdentifier::try_from(value.to_owned()).expect(
                "326a4da9 sql_identifier_uses_restricted_ascii_grammar invariant must hold",
            );
        });
        [
            str_constants::PG_CRUD_EMPTY_SQL_SUFFIX,
            str_constants::VALUE_2TABLE,
            str_constants::TABLE_NAME,
            str_constants::NON_ASCII_U_E9,
            str_constants::TABLE_NAME_ALT,
        ]
        .into_iter()
        .for_each(|value| {
            let _error = super::SqlIdentifier::try_from(value.to_owned())
                .expect_err(str_constants::F698FD6D);
        });
    }
    #[test]
    fn query_builder_accepts_only_validated_identifiers() {
        let builder = super::SqlSelectBuilder::new(
            super::SqlQualifiedIdentifier::new(
                identifier(str_constants::PUBLIC),
                identifier(str_constants::USERS_ALT),
            ),
            vec![
                identifier(str_constants::SQL_NAMES_ID),
                identifier(str_constants::LOGIN),
            ]
            .into(),
        );
        let first = builder.build();
        let second = builder.build();
        assert_eq!(first.into_inner(), "SELECT id, login FROM public.users");
        assert_eq!(second.into_inner(), "SELECT id, login FROM public.users");
    }
    #[test]
    fn benchmark_black_box_dependency_is_available() {
        assert_ne!(size_of::<criterion::Criterion>(), 0usize);
    }
}
