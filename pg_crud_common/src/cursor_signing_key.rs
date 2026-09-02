#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone,
)]
pub struct CursorSigningKey(
    bounded_types::bounded_vec::BoundedVec<
        u8,
        { constants_usize::ONE },
        { super::cursor_signing_key_maximum_length::CURSOR_SIGNING_KEY_MAXIMUM_LENGTH },
    >,
);

impl std::fmt::Debug for CursorSigningKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}

impl TryFrom<Vec<u8>> for CursorSigningKey {
    type Error = crate::cursor_signing_key_error::CursorSigningKeyError;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self)
            .map_err(|_error| crate::cursor_signing_key_error::CursorSigningKeyError::InvalidLength)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_signing_key_rejects_empty_and_oversized_values() {
        assert_eq!(
            crate::cursor_signing_key::CursorSigningKey::try_from(Vec::new()).map(drop),
            Err(crate::cursor_signing_key_error::CursorSigningKeyError::InvalidLength)
        );
        assert_eq!(
            crate::cursor_signing_key::CursorSigningKey::try_from(vec![
                constants_u8::ZERO;
                super::super::cursor_signing_key_maximum_length::CURSOR_SIGNING_KEY_MAXIMUM_LENGTH
                    + constants_usize::ONE
            ])
            .map(drop),
            Err(crate::cursor_signing_key_error::CursorSigningKeyError::InvalidLength)
        );
    }
}
