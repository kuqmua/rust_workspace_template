#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyMethod(pub(super) String);

impl TryFrom<String> for PgTableIdempotencyMethod {
    type Error = super::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(super::PgTableIdempotencyTextError::Empty);
        }
        if value.len() > super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES {
            return Err(super::PgTableIdempotencyTextError::TooLong {
                actual_bytes: super::PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: super::PgTableIdempotencyTextBytes::from(
                    super::PG_TBL_IDEMPOTENCY_TEXT_MAX_BYTES,
                ),
            });
        }
        if matches!(
            value.as_str(),
            constants_str::POST | constants_str::PATCH | constants_str::DELETE
        ) {
            Ok(Self(value))
        } else {
            Err(super::PgTableIdempotencyTextError::InvalidMethod)
        }
    }
}
