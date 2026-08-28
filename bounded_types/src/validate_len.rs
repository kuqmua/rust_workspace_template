pub(crate) fn validate_len<const MIN: usize, const MAX: usize>(
    len: crate::BoundedLen,
) -> Result<(), crate::BoundedValueError> {
    if MIN > MAX {
        Err(crate::BoundedValueError::InvalidBounds {
            min: crate::BoundedLen::from(MIN),
            max: crate::BoundedLen::from(MAX),
        })
    } else if len.get() < MIN {
        Err(crate::BoundedValueError::BelowMin {
            actual: len,
            min: crate::BoundedLen::from(MIN),
        })
    } else if len.get() > MAX {
        Err(crate::BoundedValueError::AboveMax {
            actual: len,
            max: crate::BoundedLen::from(MAX),
        })
    } else {
        Ok(())
    }
}
