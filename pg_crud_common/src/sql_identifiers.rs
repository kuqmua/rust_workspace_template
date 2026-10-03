#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
)]
pub struct SqlIdentifiers(crate::sql_identifier_list_text::SqlIdentifierListText);

impl TryFrom<Vec<crate::sql_identifier::SqlIdentifier>> for SqlIdentifiers {
    type Error =
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: Vec<crate::sql_identifier::SqlIdentifier>) -> Result<Self, Self::Error> {
        if value.len() > bounded_types::collection_max_len::COLLECTION_MAX_LEN {
            return Err(
                crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong {
                    len: value.len(),
                    max: bounded_types::collection_max_len::COLLECTION_MAX_LEN,
                },
            );
        }
        let identifiers_len = value.iter().fold(constants_usize::ZERO, |len, identifier| {
            len.saturating_add(identifier.as_ref().len())
        });
        let separators_len = value
            .len()
            .saturating_sub(constants_usize::ONE)
            .saturating_mul(constants_str::TEXT_ALT_6.len());
        let mut text = String::with_capacity(identifiers_len.saturating_add(separators_len));
        value.iter().enumerate().for_each(|(index, identifier)| {
            if index != constants_usize::ZERO {
                text.push_str(constants_str::TEXT_ALT_6);
            }
            text.push_str(identifier.as_ref());
        });
        crate::sql_identifier_list_text::SqlIdentifierListText::try_from(text).map(Self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_identifier_lists_preserve_empty_order_duplicates_and_collection_limits() {
        assert!(
            crate::sql_identifier::SqlIdentifier::try_from(constants_str::X.to_owned())
                .is_ok_and(|identifier| {
                    [0usize, 1usize, 3usize, 10_000usize, 10_001usize]
                        .into_iter()
                        .all(|count| {
                            let result = crate::sql_identifiers::SqlIdentifiers::try_from(vec![
                                identifier.clone(); count
                            ]);
                            if count > bounded_types::collection_max_len::COLLECTION_MAX_LEN {
                                return matches!(result, Err(
                                    crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong { len, max }
                                ) if len == count && max == bounded_types::collection_max_len::COLLECTION_MAX_LEN);
                            }
                            let expected = vec![constants_str::X; count]
                                .join(constants_str::TEXT_ALT_6);
                            result.is_ok_and(|list| list.get_inner().get_inner().as_str() == expected)
                        })
                })
        );
        assert!(
            [constants_str::TABLE, constants_str::X, constants_str::TABLE]
                .into_iter()
                .map(|text| crate::sql_identifier::SqlIdentifier::try_from(text.to_owned()))
                .collect::<Result<Vec<_>, _>>()
                .is_ok_and(|identifiers| {
                    crate::sql_identifiers::SqlIdentifiers::try_from(identifiers).is_ok_and(
                        |list| {
                            list.get_inner().get_inner().as_str()
                                == [constants_str::TABLE, constants_str::X, constants_str::TABLE]
                                    .join(constants_str::TEXT_ALT_6)
                        },
                    )
                })
        );
    }

    #[test]
    fn test_identifier_list_text_limit_includes_separator_bytes() {
        assert!(
            crate::sql_identifier::SqlIdentifier::try_from(constants_str::X.repeat(128usize))
                .is_ok_and(|identifier| {
                    [125usize, 126usize, 127usize].into_iter().all(|tail_length| {
                        crate::sql_identifier::SqlIdentifier::try_from(constants_str::X.repeat(tail_length))
                            .is_ok_and(|tail| {
                                let mut identifiers = vec![identifier.clone(); 8065usize];
                                identifiers.push(tail);
                                let result = crate::sql_identifiers::SqlIdentifiers::try_from(identifiers);
                                let expected_length = 1_048_450usize + tail_length;
                                if tail_length == 127usize {
                                    return matches!(result, Err(
                                        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError::TooLong { len, max }
                                    ) if len == expected_length && max == crate::pg_crud_string_wrapper_max_len::PG_CRUD_STRING_WRAPPER_MAX_LEN);
                                }
                                result.is_ok_and(|list| list.get_inner().get_inner().as_str().len() == expected_length)
                            })
                    })
                })
        );
    }
}
