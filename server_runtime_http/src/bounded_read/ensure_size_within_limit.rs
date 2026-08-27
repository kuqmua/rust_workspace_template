pub(super) const fn ensure_size_within_limit(
    size: super::BoundedReadObservedBytes,
    maximum_bytes: super::BoundedReadMaximumBytes,
) -> Result<(), super::BoundedReadError> {
    if size.0 > maximum_bytes.0 {
        Err(super::BoundedReadError::ExceedsMaximum { maximum_bytes })
    } else {
        Ok(())
    }
}
