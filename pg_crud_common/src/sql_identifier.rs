#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct SqlIdentifier(bounded_types::bounded_string::BoundedString<1usize, 128usize, false>);
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
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::Invalid,
            })
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_sql_identifier_validates_every_ascii_byte_at_each_position() {
        assert!((0u8..=127u8).all(|byte| {
            let character = char::from(byte);
            let first = crate::sql_identifier::SqlIdentifier::try_from(character.to_string());
            let valid_first = byte.is_ascii_alphabetic() || byte == b'_';
            assert_eq!(first.is_ok(), valid_first);
            let mut suffix_input = constants_str::X.to_owned();
            suffix_input.push(character);
            let suffix = crate::sql_identifier::SqlIdentifier::try_from(suffix_input.clone());
            let valid_suffix = byte.is_ascii_alphanumeric() || byte == b'_';
            assert_eq!(suffix.is_ok(), valid_suffix);
            (!valid_first
                || first.is_ok_and(|identifier| identifier.as_ref() == character.to_string()))
                && (!valid_suffix
                    || suffix.is_ok_and(|identifier| identifier.as_ref() == suffix_input))
        }));
    }

    #[test]
    fn test_sql_identifier_length_boundaries_and_empty_error() {
        assert_eq!(
            crate::sql_identifier::SqlIdentifier::try_from(String::new()),
            Err(crate::sql_identifier_error::SqlIdentifierError::Empty)
        );
        assert!([1usize, 127usize, 128usize].into_iter().all(|length| {
            let value = constants_str::X.repeat(length);
            crate::sql_identifier::SqlIdentifier::try_from(value.clone())
                .is_ok_and(|identifier| identifier.as_ref() == value)
        }));
        assert_eq!(
            crate::sql_identifier::SqlIdentifier::try_from(constants_str::X.repeat(129usize)),
            Err(crate::sql_identifier_error::SqlIdentifierError::Invalid)
        );
    }

    #[test]
    fn test_qualified_sql_identifier_preserves_each_component_without_normalization() {
        let schema = sql_identifier_fixture(&constants_str::X.to_uppercase().repeat(128usize));
        let table = sql_identifier_fixture(&constants_str::X.repeat(128usize));
        let qualified = crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
            schema.clone(),
            table.clone(),
        );
        assert_eq!(qualified.get_schema(), &schema);
        assert_eq!(qualified.get_table(), &table);
        assert_eq!(
            qualified.to_string(),
            format!(
                "{}{}{}",
                schema.as_ref(),
                constants_str::DOT,
                table.as_ref()
            )
        );
    }

    fn sql_identifier_fixture(str: &str) -> crate::sql_identifier::SqlIdentifier {
        crate::sql_identifier::SqlIdentifier::try_from(str.to_owned())
            .expect(constants_str::DIAGNOSTIC_940EB924)
    }
    #[test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository source policy requires iterator methods instead of for loops"
    )]
    fn test_sql_identifier_uses_restricted_ascii_grammar() {
        [
            constants_str::TABLE_ALT,
            constants_str::TABLE,
            constants_str::TABLE_2,
        ]
        .into_iter()
        .for_each(|value| {
            let _identifier = crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
                .expect(constants_str::DIAGNOSTIC_326A4DA9);
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
            let _error = crate::sql_identifier::SqlIdentifier::try_from(value.to_owned())
                .expect_err(constants_str::F698FD6D);
        });
    }
    #[test]
    fn test_query_builder_accepts_only_validated_identifiers() {
        let builder = crate::sql_select_builder::SqlSelectBuilder::new(
            crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                sql_identifier_fixture(constants_str::PUBLIC),
                sql_identifier_fixture(constants_str::USERS_ALT),
            ),
            crate::sql_identifiers::SqlIdentifiers::try_from(vec![
                sql_identifier_fixture(constants_str::SQL_NAMES_ID),
                sql_identifier_fixture(constants_str::LOGIN),
            ])
            .expect(constants_str::DIAGNOSTIC_C4CF723E),
        );
        assert!(matches!(
            builder.build(),
            Ok(query) if query.as_ref() == constants_str::VALUE_F0B7B783
        ));
        assert!(matches!(
            builder.build(),
            Ok(query) if query.as_ref() == constants_str::VALUE_F0B7B783
        ));
    }
    #[test]
    fn test_query_builder_preserves_oversized_query_failure() {
        let identifier = sql_identifier_fixture(&constants_str::X.repeat(128usize));
        let result =
            crate::sql_identifiers::SqlIdentifiers::try_from(vec![identifier.clone(); 8065usize])
                .map(|columns| {
                    crate::sql_select_builder::SqlSelectBuilder::new(
                        crate::sql_qualified_identifier::SqlQualifiedIdentifier::new(
                            identifier.clone(),
                            identifier,
                        ),
                        columns,
                    )
                    .build()
                });
        assert!(matches!(
            result,
            Ok(Err(
                crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
                    len,
                    max,
                }
            )) if len > max && max == crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN
        ));
    }
    #[test]
    fn test_benchmark_black_box_dependency_is_available() {
        assert_ne!(size_of::<criterion::Criterion>(), constants_usize::ZERO);
    }
}
