#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlIdentifiers(pub(crate) crate::domain_types::SqlIdentifierListText);

impl TryFrom<Vec<crate::domain_types::SqlIdentifier>> for SqlIdentifiers {
    type Error = crate::domain_types::PgCrudStringWrapperTryFromStringError;

    fn try_from(value: Vec<crate::domain_types::SqlIdentifier>) -> Result<Self, Self::Error> {
        if value.len() > bounded_types::domain_types::COLLECTION_MAX_LEN {
            return Err(
                crate::domain_types::PgCrudStringWrapperTryFromStringError::TooLong {
                    len: value.len(),
                    max: bounded_types::domain_types::COLLECTION_MAX_LEN,
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
        crate::domain_types::SqlIdentifierListText::try_from(text).map(Self)
    }
}
