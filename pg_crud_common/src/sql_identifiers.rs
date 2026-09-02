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
        value.iter().enumerate().for_each(|(idx, identifier)| {
            if idx != constants_usize::ZERO {
                text.push_str(constants_str::TEXT_ALT_6);
            }
            text.push_str(identifier.as_ref());
        });
        crate::sql_identifier_list_text::SqlIdentifierListText::try_from(text).map(Self)
    }
}
