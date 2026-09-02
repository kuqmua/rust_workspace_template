pub(super) const fn ensure_size_within_limit(
    bounded_read_observed_bytes: crate::bounded_read_observed_bytes::BoundedReadObservedBytes,
    bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
) -> Result<(), crate::bounded_read_error::BoundedReadError> {
    if bounded_read_observed_bytes.get() > bounded_read_maximum_bytes.get() {
        Err(
            crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                maximum_bytes: bounded_read_maximum_bytes,
            },
        )
    } else {
        Ok(())
    }
}
