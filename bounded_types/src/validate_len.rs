pub(crate) fn validate_len<const MIN: usize, const MAX: usize>(
    bounded_len: crate::bounded_len::BoundedLen,
) -> Result<(), crate::bounded_value_error::BoundedValueError> {
    if MIN > MAX {
        Err(
            crate::bounded_value_error::BoundedValueError::InvalidBounds {
                min: crate::bounded_len::BoundedLen::from(MIN),
                max: crate::bounded_len::BoundedLen::from(MAX),
            },
        )
    } else if bounded_len.get() < MIN {
        Err(crate::bounded_value_error::BoundedValueError::BelowMin {
            actual: bounded_len,
            min: crate::bounded_len::BoundedLen::from(MIN),
        })
    } else if bounded_len.get() > MAX {
        Err(crate::bounded_value_error::BoundedValueError::AboveMax {
            actual: bounded_len,
            max: crate::bounded_len::BoundedLen::from(MAX),
        })
    } else {
        Ok(())
    }
}
