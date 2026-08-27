#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct PgTableIdempotencyRoute(pub(super) String);

impl TryFrom<String> for PgTableIdempotencyRoute {
    type Error = super::PgTableIdempotencyTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(super::PgTableIdempotencyTextError::Empty);
        }
        if value.len() > super::PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES {
            return Err(super::PgTableIdempotencyTextError::TooLong {
                actual_bytes: super::PgTableIdempotencyTextBytes::from(value.len()),
                maximum_bytes: super::PgTableIdempotencyTextBytes::from(
                    super::PG_TBL_IDEMPOTENCY_ROUTE_MAX_BYTES,
                ),
            });
        }
        if value.starts_with('/') {
            Ok(Self(value))
        } else {
            Err(super::PgTableIdempotencyTextError::InvalidRoute)
        }
    }
}
