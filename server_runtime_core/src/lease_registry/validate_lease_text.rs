use super::{LeaseTextError, LeaseTextRef};

pub(super) fn validate_lease_text(value: LeaseTextRef<'_>) -> Result<(), LeaseTextError> {
    if value.0.is_empty() {
        Err(LeaseTextError::Empty)
    } else if value.0.contains('\0') {
        Err(LeaseTextError::ContainsNul)
    } else {
        Ok(())
    }
}
