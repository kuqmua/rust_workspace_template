#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone)]
pub struct CursorSigningKey(
    pub(super)  bounded_types::domain_types::vector::BoundedVec<
        u8,
        { constants_usize::ONE },
        { super::cursor_signing_key_maximum_length::CURSOR_SIGNING_KEY_MAXIMUM_LENGTH },
    >,
);

impl std::fmt::Debug for CursorSigningKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

impl TryFrom<Vec<u8>> for CursorSigningKey {
    type Error = crate::domain_types::CursorSigningKeyError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|_error| crate::domain_types::CursorSigningKeyError)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn signing_key_rejects_empty_and_oversized_values() {
        assert_eq!(
            super::CursorSigningKey::try_from(Vec::new()).map(drop),
            Err(crate::domain_types::CursorSigningKeyError)
        );
        assert_eq!(
            super::CursorSigningKey::try_from(vec![
                constants_u8::ZERO;
                super::super::cursor_signing_key_maximum_length::CURSOR_SIGNING_KEY_MAXIMUM_LENGTH
                    + constants_usize::ONE
            ])
            .map(drop),
            Err(crate::domain_types::CursorSigningKeyError)
        );
    }
}
