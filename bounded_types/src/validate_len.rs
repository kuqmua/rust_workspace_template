pub(super) fn validate_len<const MIN: usize, const MAX: usize>(
    len: crate::domain_types::BoundedLen,
) -> Result<(), crate::domain_types::BoundedValueError> {
    if MIN > MAX {
        Err(crate::domain_types::BoundedValueError::InvalidBounds {
            min: crate::domain_types::BoundedLen::from(MIN),
            max: crate::domain_types::BoundedLen::from(MAX),
        })
    } else if len.get() < MIN {
        Err(crate::domain_types::BoundedValueError::BelowMin {
            actual: len,
            min: crate::domain_types::BoundedLen::from(MIN),
        })
    } else if len.get() > MAX {
        Err(crate::domain_types::BoundedValueError::AboveMax {
            actual: len,
            max: crate::domain_types::BoundedLen::from(MAX),
        })
    } else {
        Ok(())
    }
}
