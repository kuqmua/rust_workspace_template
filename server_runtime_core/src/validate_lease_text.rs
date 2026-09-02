pub(super) fn validate_lease_text(
    lease_text_ref: crate::lease_text_ref::LeaseTextRef<'_>,
) -> Result<(), crate::lease_text_error::LeaseTextError> {
    if lease_text_ref.len() > crate::lease_text_maximum_bytes::LEASE_TEXT_MAXIMUM_BYTES {
        Err(crate::lease_text_error::LeaseTextError::TooLong)
    } else if lease_text_ref.is_empty() {
        Err(crate::lease_text_error::LeaseTextError::Empty)
    } else if lease_text_ref.contains('\0') {
        Err(crate::lease_text_error::LeaseTextError::ContainsNul)
    } else {
        Ok(())
    }
}
