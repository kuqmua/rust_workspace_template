#[derive(
    proc_macro_getters::Getters, proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone,
)]
pub struct CursorSigningKey(
    bounded_types::bounded_vec::BoundedVec<
        u8,
        { constants_usize::VALUE_32 },
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

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(|_error| crate::cursor_signing_key_error::CursorSigningKeyError::InvalidLength)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_signing_key_rejects_short_and_oversized_values() {
        assert_eq!(
            crate::cursor_signing_key::CursorSigningKey::try_from(vec![
                constants_u8::ZERO;
                constants_usize::VALUE_32
                    - constants_usize::ONE
            ])
            .map(drop),
            Err(crate::cursor_signing_key_error::CursorSigningKeyError::InvalidLength)
        );
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
    #[test]
    fn test_signing_key_boundaries_preserve_bytes_and_redact_debug() {
        assert!(crate::cursor_signing_key::CursorSigningKey::try_from(vec![0u8; 32usize]).is_ok_and(|reference| {
            let regular = format!("{reference:?}");
            let alternate = format!("{reference:#?}");
            regular.starts_with(std::any::type_name::<crate::cursor_signing_key::CursorSigningKey>())
                && [32usize, super::super::cursor_signing_key_maximum_length::CURSOR_SIGNING_KEY_MAXIMUM_LENGTH].into_iter().all(|length| {
                    [193u8, 251u8].into_iter().all(|byte| {
                        let input = vec![byte; length];
                        let pointer = input.as_ptr();
                        crate::cursor_signing_key::CursorSigningKey::try_from(input).is_ok_and(|cursor_signing_key| {
                            let cloned = cursor_signing_key.clone();
                            format!("{cursor_signing_key:?}") == regular
                                && format!("{cursor_signing_key:#?}") == alternate
                                && format!("{cloned:?}") == regular
                                && format!("{cloned:#?}") == alternate
                                && cursor_signing_key.get_inner().as_slice().as_ptr() == pointer
                                && cursor_signing_key.get_inner().as_slice().len() == length
                                && cursor_signing_key.get_inner().as_slice().iter().all(|value| *value == byte)
                                && cloned.get_inner().as_slice() == cursor_signing_key.get_inner().as_slice()
                        })
                    })
                })
        }));
    }
}
