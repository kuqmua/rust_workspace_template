pub(super) fn validate_lease_text(
    value: crate::lease_text_ref::LeaseTextRef<'_>,
) -> Result<(), crate::lease_text_error::LeaseTextError> {
    if value.0.len() > crate::lease_text_maximum_bytes::LEASE_TEXT_MAXIMUM_BYTES {
        Err(crate::lease_text_error::LeaseTextError::TooLong)
    } else if value.0.is_empty() {
        Err(crate::lease_text_error::LeaseTextError::Empty)
    } else if value.0.contains('\0') {
        Err(crate::lease_text_error::LeaseTextError::ContainsNul)
    } else {
        Ok(())
    }
}
