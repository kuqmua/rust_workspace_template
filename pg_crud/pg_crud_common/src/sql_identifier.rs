#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SqlIdentifier(String);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum SqlIdentifierError {
    #[error("SQL identifier is empty")]
    Empty,
    #[error("SQL identifier contains unsupported characters")]
    Invalid,
}
impl TryFrom<String> for SqlIdentifier {
    type Error = SqlIdentifierError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
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
        Ok(Self(value))
    }
}
impl AsRef<str> for SqlIdentifier {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SqlQualifiedIdentifier {
    schema: SqlIdentifier,
    table: SqlIdentifier,
}
#[derive(Debug)]
struct SqlQueryText(String);
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SqlSelectBuilder {
    columns: Vec<SqlIdentifier>,
    table: SqlQualifiedIdentifier,
}
impl SqlSelectBuilder {
    #[must_use]
    pub fn build(self) -> crate::QueryPartFragment {
        let columns_len = self
            .columns
            .iter()
            .map(|column| column.as_ref().len())
            .sum::<usize>();
        let mut query =
            SqlQueryText::try_from(String::with_capacity(columns_len.saturating_add(32usize)))
                .unwrap_or_else(|_error| SqlQueryText(String::new()));
        query.0.push_str(str_constants::SELECT);
        self.columns.iter().enumerate().for_each(|(idx, column)| {
            if idx != 0usize {
                query.0.push_str(str_constants::TEXT_ALT_6);
            }
            query.0.push_str(column.as_ref());
        });
        query.0.push_str(str_constants::FROM);
        self.table.push_to(&mut query);
        crate::QueryPartFragment::try_from(query.0).unwrap_or_else(crate::QueryPartFragment::from)
    }
    #[must_use]
    pub const fn new(table: SqlQualifiedIdentifier, columns: Vec<SqlIdentifier>) -> Self {
        Self { columns, table }
    }
}
#[cfg(test)]
mod tests {
    fn identifier(value: &str) -> super::SqlIdentifier {
        super::SqlIdentifier::try_from(value.to_owned()).expect("940eb924")
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
            let _identifier = super::SqlIdentifier::try_from(value.to_owned()).expect("326a4da9");
        });
        [
            str_constants::pg_crud::EMPTY_SQL_SUFFIX,
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
        let query = super::SqlSelectBuilder::new(
            super::SqlQualifiedIdentifier::new(
                identifier(str_constants::PUBLIC),
                identifier(str_constants::USERS_ALT),
            ),
            vec![
                identifier(str_constants::sql_names::ID),
                identifier(str_constants::LOGIN),
            ],
        )
        .build();
        assert_eq!(query.into_inner(), "SELECT id, login FROM public.users");
    }
    #[test]
    fn benchmark_black_box_dependency_is_available() {
        assert_ne!(size_of::<criterion::Criterion>(), 0usize);
    }
}
