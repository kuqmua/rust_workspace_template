pub(super) const fn ensure_size_within_limit(
    size: crate::bounded_read_observed_bytes::BoundedReadObservedBytes,
    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
) -> Result<(), crate::bounded_read_error::BoundedReadError> {
    if size.0 > maximum_bytes.0 {
        Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })
    } else {
        Ok(())
    }
}
